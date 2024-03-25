#version 460

precision mediump float;


uniform float time;
vec3 light_dir = normalize(vec3(1.0, 0.0, 2.0));

// Angle to the normal vector beyond which the light is totally reflected.
const float total_reflection_angle = 0.837758;

in vec2 vert;
out vec4 color;

float amplify(float x) {
    return x*x*x*x;
}

void main() {
    //sin(vert.x*5.0 + time)/3.0
    float derivative_x = 5.0*cos(vert.x*5.0 + time)/30.0;
    float derivative_y = 6.0*cos(vert.y*6.0 + 2.0*time)/20.0;
    vec3 normal = normalize(vec3(-derivative_x, -derivative_y, 1.0));
    float light = dot(normal, light_dir);
    light = amplify(light);

    //vec2 darker = vec2(0.24, 0.37);
    //vec2 brighter = vec2(0.64, 0.81);
    vec3 darker = vec3(0.00, 0.03, 0.23);
    vec3 brighter = vec3(0.48, 0.54, 0.96);
    vec3 choice = mix(darker, brighter, light);
    //vec2 choice = mix(darker, brighter, light);
    //vec3 darker = vec3(0.0, 0.0, 0.0);
    //vec3 brighter = vec3(1.0, 1.0, 1.0);
    //vec3 choice = mix(darker, brighter, light);
    // vec2 choice =vec2(0.0, 0.0);
    // if (light < 0.7) {
    //     choice = vec2(1.0, 1.0);
    // }
    //float angle = atan(normal.y, normal.x)/3.14159;
    color = vec4(choice , 1.0);
    //color = vec4(vert, 0.5, 1.0);
}