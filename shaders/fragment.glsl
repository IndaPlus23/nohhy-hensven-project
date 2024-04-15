#version 450 core

const int MAX_SPHERES = 100;
const int MAX_TRIANGLES = 100;
const float MIN_DIST = 0.01;
const int MAX_DEPTH = 100;

uniform int numOfSpheres;
uniform int numOfTriangles;

uniform vec3 lightPos;

// uniform vec3 cameraPos;
// uniform vec3 cameraViewDirection;
// uniform float cameraFOV;

uniform vec2 u_resolution;

out vec4 FragColor;

struct Sphere {
    vec3 pos;
    vec3 color;
    float radius;
};

struct Triangle {
    vec3 v1;
    vec3 v2;
    vec3 v3;
    vec3 norm;
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

/*
uniform SphereBuffer {
    PaddedSphere paddedSpheres[MAX_SPHERES];
};



uniform TriangleBuffer {
    PaddedTriangle paddedTriangles[MAX_TRIANGLES];
};

*/

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
    PaddedSphere paddedSphere = paddedSpheres[index];

    return Sphere(paddedSphere.pos.xyz, paddedSphere.color.xyz, paddedSphere.radius.x);
}

Triangle getTriangle(int index) {
    PaddedTriangle paddedTriangle = paddedTriangles[index];

    return newTriangle(paddedTriangle.v1.xyz, paddedTriangle.v2.xyz, paddedTriangle.v3.xyz, paddedTriangle.color.xyz);
}

float sphereDist(Sphere sphere, vec3 pos) {
    return dist3(sphere.pos, pos) - sphere.radius;
}

float dot2(vec3 v) { return dot(v,v); }

// from https://iquilezles.org/articles/distfunctions/
float triangleDist(Triangle triangle, vec3 pos) {
    vec3 a = triangle.v1;
    vec3 b = triangle.v2;
    vec3 c = triangle.v3;

    vec3 ba = b - a; 
    vec3 cb = c - b; 
    vec3 ac = a - c; 

    vec3 nor = triangle.norm;

    vec3 cr_ba_nor = cross(ba,nor);
    vec3 cr_cb_nor = cross(cb,nor);
    vec3 cr_ac_nor = cross(ac,nor);

    float dot2_ba = dot2(ba);
    float dot2_cb = dot2(cb);
    float dot2_ac = dot2(ac);
    float dot2_nor = dot2(nor);

    vec3 p = pos;
    vec3 pb = p - b;
    vec3 pa = p - a;
    vec3 pc = p - c;

    return sqrt(
    (sign(dot(cr_ba_nor,pa)) +
        sign(dot(cr_cb_nor,pb)) +
        sign(dot(cr_ac_nor,pc))<2.0)
        ?
        min( min(
        dot2(ba*clamp(dot(ba,pa)/dot2_ba,0.0,1.0)-pa),
        dot2(cb*clamp(dot(cb,pb)/dot2_cb,0.0,1.0)-pb)),
        dot2(ac*clamp(dot(ac,pc)/dot2_ac,0.0,1.0)-pc))
        :
        dot(nor,pa)*dot(nor,pa)/dot2_nor);
}

vec3 getDir(vec2 uv) {
    return vec3(uv.xy, 0.5);
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

vec3 _march(Ray ray, int depth, Sphere spheres[MAX_SPHERES], Triangle triangles[MAX_TRIANGLES]) {
    vec3 clr = vec3(0);
    float dst = 100000.0;

    while (dst > MIN_DIST) {
        dst = 100000.0;

        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = spheres[i];

            float new_dst = sphereDist(sphere, ray.pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = sphere.color * getLightCoefSphere(ray, sphere);
            }

        }

        for (int j = 0; j < numOfTriangles; j++) {
            Triangle triangle = triangles[j];

            float new_dst = triangleDist(triangle, ray.pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = triangle.color * getLightCoefTriangle(ray, triangle);
            }
        }

        if (depth <= 0) {
            return vec3(0.5);
        }

        ray = Ray(ray.pos + ray.dir * dst, ray.dir);
        depth -= 1;
    }

    //vec3 glow = vec3(float(MAX_DEPTH - depth) / float(MAX_DEPTH));

    return clr;
}

vec3 rayMarch(vec2 uv, vec3 origin, Sphere spheres[MAX_SPHERES], Triangle triangles[MAX_TRIANGLES]) {
    vec3 dir = getDir(uv);
    dir = normalize(dir);

    Ray ray = Ray(origin, dir);

    return _march(ray, MAX_DEPTH, spheres, triangles);
}

void main() {
    // Screen coordinates normalized to [-1, 1]
    vec2 uv = 2.0 * gl_FragCoord.xy / u_resolution.xy - 1.0;

    // TODO: transform ray_dir depending on camera position
    vec2 ray_dir = uv;
    // TODO: move origin to camera position
    vec3 origin = vec3(0);

    // spheres array contains all spheres loaded in from the paddedSpheres UBO
    Sphere spheres [MAX_SPHERES];
    for (int i = 0; i < numOfSpheres; i++) {
        spheres[i] = getSphere(i);
    } 

    Triangle triangles [MAX_TRIANGLES];
    for (int i = 0; i < numOfTriangles; i++) {
        triangles[i] = getTriangle(i);
    } 

    // FragColor = vec4(1.0);

    FragColor = vec4(rayMarch(ray_dir, origin, spheres, triangles), 1.0);
}
