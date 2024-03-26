#version 330 core

const int MAX_SPHERES = 100;
const float MIN_DIST = 0.001;
const int MAX_DEPTH = 1000;

uniform int numOfSpheres;

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

struct PaddedSphere {
    vec4 pos;
    vec4 color;
    vec4 radius;
};

struct Ray {
    vec3 pos;
    vec3 dir;
};

uniform SphereBuffer {
    PaddedSphere paddedSpheres[100];
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

vec3 _march(Ray ray, int depth, Sphere spheres[MAX_SPHERES]) {
    float dst = 10000000.0;
    vec3 clr = vec3(0);

    while (dst > MIN_DIST) {
        for (int i = 0; i < numOfSpheres; i++) {
            Sphere sphere = spheres[i];

            float new_dst = sphereDist(sphere, ray.pos) - sphere.radius;

            if (new_dst < dst) {
                dst = new_dst;
                clr = sphere.color * getLightCoefSphere(ray, sphere);
            }

        }

        if (depth <= 0) {
            return vec3(0.5);
        }

        ray = Ray(ray.pos + ray.dir * dst, ray.dir);
        depth -= 1;
    }

    return clr;
}

vec3 rayMarch(vec2 uv, vec3 origin, Sphere spheres[MAX_SPHERES]) {
    vec3 dir = getDir(uv);
    dir = normalize(dir);

    Ray ray = Ray(origin, dir);

    return _march(ray, MAX_DEPTH, spheres);
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

    FragColor = vec4(rayMarch(ray_dir, origin, spheres), 1.0);
}
