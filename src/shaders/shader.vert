#version 460

const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);
uniform float time;
uniform mat4 camera;

in vec2 in_vert;
out vec2 vert;

void main() {
    mat2 rot = mat2(
        cos(time), -sin(time),
        sin(time), cos(time)
    );
    vert = in_vert - 0.5;
    //vert = rot * vert;
    gl_Position = camera *  vec4(vert, sin(vert.x*5.0 + time)/3.0, 1.0);
}