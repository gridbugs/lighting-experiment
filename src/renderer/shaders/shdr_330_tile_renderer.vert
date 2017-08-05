#version 330 core

in vec2 a_Pos;
in vec2 a_Tex;

out vec2 v_Tex;

void main() {
    v_Tex = vec2(a_Tex.x, 1.0 - a_Tex.y);;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
