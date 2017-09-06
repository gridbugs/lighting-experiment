#version 150 core

uniform Locals {
    vec2 u_InTexSize;
    vec2 u_OutTexSize;
};

in vec2 a_Pos;

in vec2 a_InPixPos;
in vec2 a_OutPixPos;
in vec2 a_PixSize;
in float a_Depth;

out vec2 v_TexCoord;

void main() {

    vec2 in_pix = a_InPixPos + a_Pos * a_PixSize;
    vec2 out_pix = a_OutPixPos + a_Pos * a_PixSize;

    v_TexCoord = in_pix / u_InTexSize;

    vec2 out_scaled = out_pix / u_OutTexSize;
    vec2 dst = vec2(out_scaled.x * 2.0 - 1.0, 1.0 - out_scaled.y * 2.0);
    gl_Position = vec4(dst, a_Depth, 1.0);
}
