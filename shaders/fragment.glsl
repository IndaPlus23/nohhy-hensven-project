#version 450 core

const int MAX_SPHERES = 100;
const int MAX_BOXES = 100;
const float MIN_DIST = 0.001;
const int MAX_DEPTH = 100;

uniform int numOfSpheres;
uniform int numOfBoxes;

// 0 : Normal
// 1 : Intersect
uniform int renderMode;

uniform vec3 lightPos;

uniform vec3 cameraPos;
uniform vec4 cameraRotationQuaternion;
uniform float cameraFOV;

uniform vec2 u_resolution;

out vec4 FragColor;

struct Sphere {
    vec3 pos;
    vec3 color;
    float radius;
};

struct Box {
    vec3 pos;
    vec3 dim;
    vec3 color;
};

struct PaddedSphere {
    vec4 pos;
    vec4 color;
    vec4 radius;
};

struct PaddedBox {
    vec4 pos;
    vec4 dim;
    vec4 color;
};

struct Ray {
    vec3 pos;
    vec3 dir;
};

/// SSBO 
layout(std430, binding = 10) buffer spheres_
{
    PaddedSphere paddedSpheres[MAX_SPHERES];
};

layout(std430, binding = 11) buffer boxes_
{
    PaddedBox paddedBoxes[MAX_BOXES];
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

Box getBox(int index) {
    PaddedBox paddedBox = paddedBoxes[index];

    return Box(paddedBox.pos.xyz, paddedBox.dim.xyz, paddedBox.color.xyz);
}

float sphereDist(Sphere sphere, vec3 pos) {
    return dist3(sphere.pos, pos) - sphere.radius;
}

// from https://iquilezles.org/articles/distfunctions/
float boxDist(Box box, vec3 pos)
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


vec3 floorColor(float x, float z) {
    vec3 clr;

    if (int(floor(x) + floor(z)) % 2 == 0) {
        clr = vec3(0.5);
    } else {
        clr = vec3(0);
    }

    clr = 2.0 * clr / dist3(cameraPos, vec3(x, 0, z));

    return clr;
}

vec4 minDist(vec3 pos, Sphere spheres[MAX_SPHERES], Box boxes[MAX_BOXES]) {
    vec3 clr;
    float dst;

    if (renderMode == 0) {
        dst = 10000000.0;

        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = spheres[i];

            float new_dst = sphereDist(sphere, pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = sphere.color;
            }
        }

        for (int i = 0; i < numOfBoxes; i++) {
            Box box = boxes[i];

            float new_dst = boxDist(box, pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = box.color;
            }
        }
    } else if (renderMode == 1) {
        dst = 0.0;

        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = spheres[i];

            float new_dst = sphereDist(sphere, pos);

            if (new_dst > dst) {
                dst = new_dst;
                clr = sphere.color;
            }
        }

        for (int i = 0; i < numOfBoxes; i++) {
            Box box = boxes[i];

            float new_dst = boxDist(box, pos);

            if (new_dst > dst) {
                dst = new_dst;
                clr = box.color;
            }
        }  
    }

    return vec4(dst, clr);
}

vec3 approxNorm(vec3 pos, float dst, Sphere spheres[MAX_SPHERES], Box boxes[MAX_BOXES]) {
    float dx = dst - minDist(pos + vec3(0.0001, 0.0, 0.0), spheres, boxes).x;
    float dy = dst - minDist(pos + vec3(0.0, 0.0001, 0.0), spheres, boxes).x;
    float dz = dst - minDist(pos + vec3(0.0, 0.0, 0.0001), spheres, boxes).x;

    return normalize(vec3(dx, dy, dz));
}

vec3 shade(vec3 clr, vec3 norm, vec3 pos) {
    return clr * dot(getLightVec(pos), norm);
}

vec3 _march(Ray ray, int depth, Sphere spheres[MAX_SPHERES], Box boxes[MAX_BOXES]) {
    float dst = 1000000.0;
    vec3 clr = vec3(0);
    vec3 p;

    while (dst > MIN_DIST) {
        p = ray.pos; 

        vec4 drgb = minDist(p, spheres, boxes);

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
            return vec3(0.5);
        }

        ray = Ray(ray.pos + ray.dir * dst, ray.dir);
        depth -= 1;
    }

    return shade(clr, approxNorm(p, dst, spheres, boxes), p);;
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

vec3 rayMarch(vec2 uv, vec3 origin, Sphere spheres[MAX_SPHERES], Box boxes[MAX_BOXES]) {
    vec3 dir = getDir(uv);
    dir = rotateDir(dir);
    dir = normalize(dir);

    Ray ray = Ray(origin, dir);

    return _march(ray, MAX_DEPTH, spheres, boxes);
}

void main() {
    // Screen coordinates normalized to [-1, 1]
    vec2 uv = 2.0 * gl_FragCoord.xy / u_resolution.xy - 1.0;

    // TODO: transform ray_dir depending on camera position
    vec2 ray_dir = uv;
    
    vec3 origin = cameraPos;

    Sphere spheres [MAX_SPHERES];
    for (int i = 0; i < numOfSpheres; i++) {
        spheres[i] = getSphere(i);
    } 

    Box boxes [MAX_BOXES];
    for (int i = 0; i < numOfBoxes; i++) {
        boxes[i] = getBox(i);
    } 

    FragColor = vec4(rayMarch(ray_dir, origin, spheres, boxes), 1.0);
}