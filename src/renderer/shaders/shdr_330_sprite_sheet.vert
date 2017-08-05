#version 330 core

uniform Locals {
    vec2 u_InStep;
    vec2 u_OutStep;
    vec2 u_TexSize;
};

in vec2 a_Pos;

in vec2 a_TexOffset;
in float a_Index;

out vec2 v_TexCoord;

void main() {

    vec2 normalised_pos = vec2((a_Pos.x + 1.0) / 2.0,
                               (1.0 - a_Pos.y) / 2.0);

    vec2 tex_base = a_TexOffset / u_TexSize;
    v_TexCoord = tex_base + normalised_pos * u_InStep;

    vec2 normalised_dst_base = vec2(a_Index * u_OutStep.x, 0.0);
    vec2 normalised_dst = normalised_dst_base + normalised_pos * u_OutStep;
    vec2 dst = vec2(normalised_dst.x * 2.0 - 1.0,
                    1.0 - normalised_dst.y * 2.0);

    gl_Position = vec4(dst, 0.0, 1.0);
}
