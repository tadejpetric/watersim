#version 460

uniform float time;
uniform mat4 camera;
uniform mat4 perspective;

const int iter = 3;
uniform float num_params;
uniform float a[iter];
uniform float b[iter];
uniform float c[iter];
uniform float d[iter];

in vec2 vert;
out vec2 vert_vs;
out vec3 water_surface;

vec3 gerstner(float x, float y, float time, float amplitude, float x_dir, float y_dir, float time_scale) {
    float res_x = amplitude*sin(x*x_dir + y*y_dir -time*time_scale);
    float res_y = amplitude*sin(x*x_dir + y*y_dir -time*time_scale);
    float res_z = amplitude*cos(x*x_dir + y*y_dir -time*time_scale);
    return vec3(res_x, res_y, res_z);
}


void main() {
    water_surface = vec3(0.0, 0.0, 0.0);
    for (int i = 0; i < iter; i++) {
       water_surface += gerstner(vert.x, vert.y, time, a[i], b[i], c[i], d[i]);
    }

    water_surface.x += vert.x;
    water_surface.y += vert.y;

    gl_Position = perspective * camera * vec4(water_surface, 1.0);
    vert_vs = vert;
}