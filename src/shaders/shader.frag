#version 460

precision mediump float;
const int iter = 32;
uniform float num_params;
uniform float a[iter];
uniform float b[iter];
uniform float c[iter];
uniform float d[iter];

uniform float time;
uniform vec3 camera_pos;

vec3 light_dir = normalize(vec3(-50.0, -50.0, 5.0));
const vec3 darker = vec3(0.00, 0.03, 0.23);
const vec3 brighter = vec3(0.48, 0.54, 0.96);

const vec3 sun_colour = vec3(1.0, 1.0, 1.0);
const float sun_size_full = 0.2;
const float sun_size_min = 0.1;
const float sun_intensity = 0.9;
// Angle to the normal vector beyond which the light is totally reflected.
const float total_reflection_angle = 0.837758;

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
    float partial_x = 0.0;
    float partial_y = 0.0;

    for (int i = 0; i < iter; i++) {
        partial_x += a[i] * b[i] * cos(b[i] * water_surface.x + c[i] * water_surface.y + d[i] * time);
        partial_y += a[i] * c[i] * cos(b[i] * water_surface.x + c[i] * water_surface.y + d[i] * time);
    }

    vec3 normal = normalize(vec3(-partial_x, -partial_y, 1.0));
    
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