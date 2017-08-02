#version 330 core

in vec2 a_Pos;
in vec4 a_Col;
in vec2 a_Translate;

out vec4 v_Col;

void main() {
    v_Col = a_Col;

    vec2 shifted = a_Pos + a_Translate;
    gl_Position = vec4(shifted.x * 2.0 - 1.0, 1.0 - shifted.y * 2.0, 0.0, 1.0);
}
