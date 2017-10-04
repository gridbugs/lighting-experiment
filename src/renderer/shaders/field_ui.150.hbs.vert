#version 150 core

{{INCLUDE_VISION}}
{{INCLUDE_DIMENSIONS}}
{{INCLUDE_SCROLL_OFFSET}}

in vec2 a_Pos;

void main() {
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
