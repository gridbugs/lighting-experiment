#version 150 core

in vec2 a_Pos;
in vec2 a_Texel;

out vec2 v_Texel;

void main() {
    v_Texel = a_Texel;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
