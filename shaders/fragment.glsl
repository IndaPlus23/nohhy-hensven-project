#version 330 core

const int MAX_SPHERES = 100;
const int MAX_TRIANGLES = 100;
const float MIN_DIST = 0.001;
const int MAX_DEPTH = 1000;

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

        for (int j = 0; j < 1; j++) {
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

    triangles[0] = newTriangle(vec3(-1.5, -1.5, 1.0), vec3(-1.5, 1.5, 1.0), vec3(1.5, -1.5, 1.0), vec3(0.8078, 0.1647, 0.3569));

    FragColor = vec4(rayMarch(ray_dir, origin, spheres, triangles), 1.0);
}