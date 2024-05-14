#version 450 core


in vec3 vColor;
out vec4 f_color;

const int MAX_SPHERES = 100;
const int MAX_TRIANGLES = 100;
const float MIN_DIST = 0.001;
const int MAX_DEPTH = 100;

const vec3 BG_CLR = vec3(0.6, 0.6, 0.6);

uniform int numOfSpheres;
uniform int numOfTriangles;
uniform int numOfBoxes;
uniform vec3 lightPos;

// 0 : Normal
// 1 : Intersect
uniform int renderMode;
uniform float smoothness;

uniform vec3 cameraPos;
uniform vec4 cameraRotationQuaternion;
uniform float cameraFOV;

uniform vec2 u_resolution;

struct Sphere {
    vec3 pos;
    vec3 color;
    float radius;
};


layout(std140) buffer sphere_array {
    float number_of_objects;
    vec4 positions[128];
    vec4 colors[128];
    vec4 radius[128];
};

layout(std140) buffer triangle_array {
    vec4 v1[128];
    vec4 v2[128];
    vec4 v3[128];
    vec4 norm[128];
    vec4 color_triangles[128];
};

layout(std140) buffer cube_array {
    vec4 pos_cubes[128];
    vec4 dim_cubes[128];
    vec4 color_cubes[128];
};

Sphere getSphereFromIndex(int id){
    Sphere s; 
    s.radius = radius[id].x;
    s.pos = vec3(positions[id].x, positions[id].y, positions[id].z);
    s.color = vec3(colors[id].x, colors[id].y, colors[id].z);
    return s;
}

struct Triangle {
    vec3 v1;
    vec3 v2;
    vec3 v3;
    vec3 norm;
    vec3 color;
};

struct Cube {
    vec3 pos;
    vec3 dim;
    vec3 color;
};

Triangle newTriangle(vec3 a, vec3 b, vec3 c, vec3 clr) {
    vec3 ba = b - a;
    vec3 ac = a - c;
    vec3 nor = normalize(cross( ba, ac ));
    
    return Triangle(a, b, c, nor, clr);
}


struct PaddedSphere {
    vec4 pos;
    vec4 color;
    vec4 radius;
};

struct PaddedTriangle {
    vec4 v1;
    vec4 v2;
    vec4 v3;
    vec4 color;
};


struct Ray {
    vec3 pos;
    vec3 dir;
};

// from old demo code, 
/// SSBO 
layout(std430, binding = 10) buffer spheres_
{
    PaddedSphere paddedSpheres[MAX_SPHERES];
};

/// SSBO 
layout(std430, binding = 11) buffer triangles_
{
    PaddedTriangle paddedTriangles[MAX_TRIANGLES];
};



float dist3(vec3 pos1, vec3 pos2) {
    float dx = pos1.x - pos2.x;
    float dy = pos1.y - pos2.y;
    float dz = pos1.z - pos2.z;

    return sqrt(dx*dx + dy*dy + dz*dz);
}


Sphere getSphere(int index) {
    Sphere s; 
    s.radius = radius[index].x;
    s.pos = vec3(positions[index].x, positions[index].y, positions[index].z);
    s.color = vec3(colors[index].x, colors[index].y, colors[index].z);
    return s;
}

Triangle getTriangle(int index) {
    Triangle t; 
    t.v1 = vec3(v1[index].x, v1[index].y, v1[index].z);
    t.v2 = vec3(v2[index].x, v2[index].y, v2[index].z);
    t.v1 = vec3(v3[index].x, v3[index].y, v3[index].z);
    t.norm = vec3(norm[index].x, norm[index].y, norm[index].z);
    t.color = vec3(color_triangles[index].x, color_triangles[index].y, color_triangles[index].z);
    return t;
}

Cube getCube(int index) {
    Cube c; 
    c.pos = vec3(pos_cubes[index].x, pos_cubes[index].y, pos_cubes[index].z);
    c.dim = vec3(dim_cubes[index].x, dim_cubes[index].y, dim_cubes[index].z);
    c.color = vec3(color_cubes[index].x, color_cubes[index].y, color_cubes[index].z);

    return c;
}

// from https://www.youtube.com/watch?v=Cp5WWtMoeKg
float smoothMin(float dstA, float dstB, float k) {
    float h = max(k - abs(dstA - dstB), 0.0) / k;
    return min(dstA, dstB) - h*h*h*k/6.0;
}

float sphereDist(Sphere sphere, vec3 pos) {
    return dist3(sphere.pos, pos) - sphere.radius;
}

float dot2(vec3 v ) { return dot(v,v); }

// from https://iquilezles.org/articles/distfunctions/
float triangleDist(Triangle triangle, vec3 pos) {
    vec3 a = triangle.v1;
    vec3 b = triangle.v2;
    vec3 c = triangle.v3;
    vec3 p = pos;

    vec3 ba = b - a; vec3 pa = p - a;
    vec3 cb = c - b; vec3 pb = p - b;
    vec3 ac = a - c; vec3 pc = p - c;
    vec3 nor = triangle.norm;

    return sqrt(
    (sign(dot(cross(ba,nor),pa)) +
        sign(dot(cross(cb,nor),pb)) +
        sign(dot(cross(ac,nor),pc))<2.0)
        ?
        min( min(
        dot2(ba*clamp(dot(ba,pa)/dot2(ba),0.0,1.0)-pa),
        dot2(cb*clamp(dot(cb,pb)/dot2(cb),0.0,1.0)-pb) ),
        dot2(ac*clamp(dot(ac,pc)/dot2(ac),0.0,1.0)-pc) )
        :
        dot(nor,pa)*dot(nor,pa)/dot2(nor) );
}


// from https://iquilezles.org/articles/distfunctions/
float cubeDist(Cube box, vec3 pos)
{
    vec3 p = pos - box.pos;
    vec3 q = abs(p) - box.dim;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

vec3 getDir(vec2 uv) {
    return vec3(uv.xy, 1.0 / tan(radians(cameraFOV / 2.0)));
}

vec3 getSphereNormal(Sphere sphere, vec3 pos) {
    return normalize((sphere.pos - pos));
}

vec3 getLightVec(vec3 pos) {
    return normalize((pos - lightPos));
}

float getLightCoefSphere(Ray ray, Sphere sphere) {
    return dot(getLightVec(ray.pos), getSphereNormal(sphere, ray.pos));
}

float getLightCoefTriangle(Ray ray, Triangle triangle) {
    return dot(getLightVec(ray.pos), triangle.norm);
}

vec3 intersectXZPlane(Ray ray) {
    // Check if the ray is parallel to the x-z plane
    if (abs(ray.dir.y) < 1e-6) {
        return vec3(0.0, 0.0, 0.0);  // Ray is parallel to the plane
    }

    float t = -ray.pos.y / ray.dir.y;

    return ray.pos + t * ray.dir;
}

float distToInt(float number) {
    float nearest_int = round(number);
    return abs(number - nearest_int);
}

vec3 vecPow(vec3 v, int x) {
    return vec3(
        pow(v.x, x),
        pow(v.y, x),
        pow(v.z, x)
    );
}

float sigmoid(float x, float k) {
    return 1.0 / (1 + exp(-k*x));
}

vec3 floorColor(float x, float z) {
    vec3 clr;

    if (int(floor(x) + floor(z)) % 2 == 0) {
        clr = vec3(0.8);
    } else {
        clr = vec3(0.6);
    }

    clr = mix(clr, BG_CLR, sigmoid(dist3(vec3(0.0, 0.0, 0.0), vec3(x, 0, z)), 0.05));

    return clr;
}


vec2 calculateColorBlending(float dist2, float dist1){

    float coeff = dist1 / dist2;

    float c1 = 2/(coeff + 1);
    float c2 = -c1 + 2;

    return vec2(c1, c2);
}

vec4 Blend(float a, float b, vec3 colA, vec3 colB, float k )
{
    float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );
    float blendDst = (a-b) * h + b - k*h*(1.0-h);
    vec3 blendCol = (colA-colB) * h + colB;
    return vec4(blendCol, blendDst);
}



vec4 minDist(vec3 pos) {
    vec3 clr;
    float dst;

    if (renderMode == 0) {
        dst = 10000000.0;

        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = getSphere(i);
            float new_dst = sphereDist(sphere, pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = sphere.color;
            }
        }

        for (int i = 0; i < numOfBoxes; i++) {
            Cube box = getCube(i);

            float new_dst = cubeDist(box, pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = box.color;
            }
        }
    } else if (renderMode == 1) {

        // some tests for boolean operators
        dst = 0.0;

        Sphere sphere = getSphere(0);
        Cube box = getCube(0);
        {
            dst = max(-sphereDist(sphere, pos), cubeDist(box, pos));
            dst = max(dst, sphereDist(getSphere(1), pos));

        }
        clr = box.color;
    } else if (renderMode == 2) {
        dst = 10000000.0;
        
        float previous_shortest_non_smooth_dist = 10000000.0;
        vec3 color_previous_shortest_object = vec3(1.0);

        for (int i = 0; i < numOfBoxes; i++) {
            Cube box = getCube(i);

            float new_dst = cubeDist(box, pos);
            float s_dst = smoothMin(dst, new_dst, smoothness);

            // for color blending
            vec2 color_coeffs = calculateColorBlending(previous_shortest_non_smooth_dist, new_dst);
            clr = Blend(previous_shortest_non_smooth_dist, new_dst, clr, box.color, 0.5).xyz;
            previous_shortest_non_smooth_dist = Blend(previous_shortest_non_smooth_dist, new_dst, color_previous_shortest_object, box.color, 0.5).w;

            if (s_dst < dst) {
                dst = s_dst;
            }
            dst = s_dst;
        }  

        
        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = getSphere(i);

            float new_dst = sphereDist(sphere, pos);
            float s_dst = smoothMin(dst, new_dst, smoothness);

            // for color blending
            vec2 color_coeffs = calculateColorBlending(previous_shortest_non_smooth_dist, new_dst);
            clr = Blend(previous_shortest_non_smooth_dist, new_dst, clr, sphere.color, 0.5).xyz;
            previous_shortest_non_smooth_dist = Blend(previous_shortest_non_smooth_dist, new_dst, color_previous_shortest_object, sphere.color, 0.5).w;

            dst = s_dst;
            color_previous_shortest_object = sphere.color;

            color_previous_shortest_object = sphere.color;
        }    
    }

    return vec4(dst, clr);
}

vec3 approxNorm(vec3 pos, float dst) {
    float dx = dst - minDist(pos + vec3(0.0001, 0.0, 0.0)).x;
    float dy = dst - minDist(pos + vec3(0.0, 0.0001, 0.0)).x;
    float dz = dst - minDist(pos + vec3(0.0, 0.0, 0.0001)).x;

    return normalize(vec3(dx, dy, dz));
}

vec3 shade(vec3 clr, vec3 norm, vec3 pos) {
    vec3 lightDir = normalize(getLightVec(pos));

    vec3 c1 = clr * (1.0 + dot(lightDir, norm)) / 2.0;
    vec3 c2 = clr *  max(dot(lightDir, norm), 0.0);

    return mix(c1, c2, 0.0);
}


vec3 _march(Ray ray, int depth) {
    float dst = 1000000.0;
    vec3 clr = vec3(0);
    vec3 p;

    while (dst > MIN_DIST) {
        p = ray.pos; 

        vec4 drgb = minDist(p);

        dst = drgb.x;
        clr = drgb.yzw;

        if (ray.pos.y < 0) {
            vec3 intersect = intersectXZPlane(ray);

            float density = 5.0;

            return floorColor(density * intersect.x, density * intersect.z);
        } 
        
        if (depth <= 0) {
            if (ray.dir.y < 0) {
                vec3 intersect = intersectXZPlane(ray);

                float density = 5.0;

                return floorColor(density * intersect.x, density * intersect.z);
            } 
            return BG_CLR;
        }

        ray = Ray(ray.pos + ray.dir * dst, ray.dir);
        depth -= 1;
    }

    return shade(clr, approxNorm(p, dst), p);
}

// Quaternion Multiplication
vec4 qMul(vec4 r, vec4 s) {
    float x = r.x * s.x - r.y * s.y - r.z * s.z - r.w * s.w;
    float y = r.x * s.y + r.y * s.x - r.z * s.w + r.w * s.z;
    float z = r.x * s.z + r.y * s.w + r.z * s.x - r.w * s.y;
    float w = r.x * s.w - r.y * s.z + r.z * s.y + r.w * s.x;

    return vec4(x, y, z, w);
}

// Descirbed in https://danceswithcode.net/engineeringnotes/quaternions/quaternions.html
vec3 rotateDir(vec3 ray_dir) {
    vec4 p = vec4(0.0, ray_dir);
    vec4 q = cameraRotationQuaternion;
    vec4 q_inv = vec4(q.x, -q.y, -q.z, -q.w);
    
    vec4 res = qMul(qMul(q_inv, p), q);

    return res.yzw;
}

vec3 rayMarch(vec2 uv, vec3 origin) {
    vec3 dir = getDir(uv);
    dir = rotateDir(dir);
    dir = normalize(dir);

    Ray ray = Ray(origin, dir);

    return _march(ray, MAX_DEPTH);
}

void main() {
    // Screen coordinates normalized to [-1, 1]
    vec2 uv = 2.0 * gl_FragCoord.xy / u_resolution.xy - 1.0;

    vec2 ray_dir = uv;
    
    vec3 origin = cameraPos;

    f_color = vec4(rayMarch(ray_dir, origin), 1.0);
}