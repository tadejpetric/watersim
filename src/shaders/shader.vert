#version 460

uniform float time;
uniform mat4 camera;
uniform mat4 perspective;

in vec2 in_vert;
out vec2 vert;

void main() {
    vert = in_vert - 0.5;
    float z = sin(vert.x*5.0 + time)/30.0;
    z += sin(vert.y*6.0 + 2.0*time)/20.0;
    gl_Position = perspective * camera * vec4(vert, z, 1.0);

}