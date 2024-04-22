#version 450 core

const int MAX_SPHERES = 100;
const int MAX_TRIANGLES = 100;
const float MIN_DIST = 0.001;
const int MAX_DEPTH = 100;

uniform int numOfSpheres;
uniform int numOfTriangles;

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

struct Ray {
    vec3 pos;
    vec3 dir;
};

/// SSBO 
layout(std430, binding = 10) buffer spheres_
{
    PaddedSphere paddedSpheres[MAX_SPHERES];
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

vec3 _march(Ray ray, int depth, Sphere spheres[MAX_SPHERES]) {
    float dst = 10000000.0;
    vec3 clr = vec3(0);

    while (dst > MIN_DIST) {
        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = spheres[i];

            float new_dst = sphereDist(sphere, ray.pos);

            if (new_dst < dst) {
                dst = new_dst;
                clr = sphere.color * getLightCoefSphere(ray, sphere);
            }
        }

        Box box = Box(vec3(1.0, 0.0, 0.0), vec3(1.0, 1.5, 0.3), vec3(0.8275, 0.3255, 0.502));

        float new_dst = boxDist(box, ray.pos);

        if (new_dst < dst) {
            dst = new_dst;
            clr = box.color;
        }

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

    return clr;
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

vec3 rayMarch(vec2 uv, vec3 origin, Sphere spheres[MAX_SPHERES]) {
    vec3 dir = getDir(uv);
    dir = rotateDir(dir);
    dir = normalize(dir);

    Ray ray = Ray(origin, dir);

    return _march(ray, MAX_DEPTH, spheres);
}

void main() {
    // Screen coordinates normalized to [-1, 1]
    vec2 uv = 2.0 * gl_FragCoord.xy / u_resolution.xy - 1.0;

    // TODO: transform ray_dir depending on camera position
    vec2 ray_dir = uv;
    
    vec3 origin = cameraPos;

    // spheres array contains all spheres loaded in from the paddedSpheres UBO
    Sphere spheres [MAX_SPHERES];
    for (int i = 0; i < numOfSpheres; i++) {
        spheres[i] = getSphere(i);
    } 

    // vec3 before = vec3(0.0, 0.0, 0.5);
    // vec3 after = rotateDir(before);

    // if (sqrt(before.x * before.x + before.y * before.y + before.z * before.z) - sqrt(after.x * after.x + after.y * after.y + after.z * after.z) > 0.0001) {
    //     FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    // } else {
    //     FragColor = vec4(abs(after), 1.0);
    // }

    FragColor = vec4(rayMarch(ray_dir, origin, spheres), 1.0);
}