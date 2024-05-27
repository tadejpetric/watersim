#version 460

precision mediump float;
const int iter = 3;
uniform float num_params;
uniform float a[iter];
uniform float b[iter];
uniform float c[iter];
uniform float d[iter];

uniform float time;
uniform vec3 camera_pos;

vec3 light_dir = normalize(vec3(-20.0, -20.0, 5.0));
//const vec3 darker = vec3(0.00, 0.03, 0.23);
const vec3 darker = vec3(0.28, 0.33, 0.73);
const vec3 brighter = vec3(0.48, 0.54, 0.96);

const vec3 sun_colour = vec3(1.0, 1.0, 1.0);
const float sun_size_full = 0.3;
const float sun_size_min = 0.1;
const float sun_intensity = 0.9;
// Angle to the normal vector beyond which the light is totally reflected.
const float total_reflection_angle = 0.837758;

in vec2 vert_vs;
in vec3 water_surface;
out vec4 color;

float amplify(float x) {
    return x;
}

vec3 project(vec3 a, vec3 b) {
    // Assuming b is normalized.
    return dot(a, b) * b;
}

float soft_sun(float angle) {
    return 1.0 - smoothstep(sun_size_min, sun_size_full, angle);
}

void main() {
    vec3 partial_x = vec3(1.0, 0.0, 0.0);
    vec3 partial_y = vec3(0.0, 1.0, 0.0);

    for (int i = 0; i < iter; i++) {
        partial_x.x += a[i] * b[i] * cos(b[i] * vert_vs.x + c[i] * vert_vs.y + d[i] * time);
        partial_x.y += a[i] * b[i] * cos(b[i] * vert_vs.x + c[i] * vert_vs.y + d[i] * time);
        partial_x.z -= a[i] * b[i] * sin(b[i] * vert_vs.x + c[i] * vert_vs.y + d[i] * time);

        partial_x.x += a[i] * c[i] * cos(b[i] * vert_vs.x + c[i] * vert_vs.y + d[i] * time);
        partial_x.y += a[i] * c[i] * cos(b[i] * vert_vs.x + c[i] * vert_vs.y + d[i] * time);
        partial_x.z -= a[i] * c[i] * sin(b[i] * vert_vs.x + c[i] * vert_vs.y + d[i] * time);    
    }

    vec3 normal = normalize(cross(partial_x, partial_y));
    
    // Water shading
    float light = dot(normal, light_dir);
    light = amplify(light);


    vec3 shaded_colour = mix(darker, brighter, light);

    // Sun reflection.
    vec3 view_dir = normalize(camera_pos - water_surface);
    float angle_to_normal = acos(dot(normal, view_dir));
    if (angle_to_normal > total_reflection_angle) {
        vec3 reflected_view = normalize(view_dir - 2.0 * project(view_dir, normal));
        float angle_to_sun = acos(dot(reflected_view, light_dir));
        float intensity = soft_sun(angle_to_sun);
        shaded_colour = mix(shaded_colour, sun_colour, intensity);

    }
    
    color = vec4(shaded_colour , 1.0);
    //color = vec4(vert, 0.5, 1.0);
}