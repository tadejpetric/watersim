#version 460

uniform float time;
uniform mat4 camera;
uniform mat4 perspective;

const int iter = 32;
uniform float num_params;
uniform float a[iter];
uniform float b[iter];
uniform float c[iter];
uniform float d[iter];

in vec2 vert;
out vec3 water_surface;

void main() {
    float z = 0.0;
    for (int i = 0; i < iter; i++) {
        z += a[i]* sin(b[i] * vert.x + c[i] * vert.y + d[i] * time);
    }
    water_surface = vec3(vert.x, vert.y, z);
    gl_Position = perspective * camera * vec4(vert, z, 1.0);

}