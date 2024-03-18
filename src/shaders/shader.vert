#version 460

const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);


uniform float time;
uniform mat4 camera;
uniform mat4 perspective;

in vec2 in_vert;
out vec2 vert;

void main() {
    mat2 rot = mat2(
       cos(time), -sin(time),
       sin(time), cos(time)
    );
    vert = in_vert - 0.5;
    //vert = rot * vert;
    float z = sin(vert.x*5.0 + time)/30.0;
    z += sin(vert.y*6.0 + 2.0*time)/20.0;
    gl_Position = perspective * camera * vec4(vert, z, 1.0);

}