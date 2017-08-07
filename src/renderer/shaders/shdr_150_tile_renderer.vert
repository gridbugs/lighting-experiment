#version 150 core

uniform Locals {
    vec2 u_OutputSizePix;
    vec2 u_SpriteSize;
};

in vec2 a_Pos;

in float a_SpriteIndex;
in vec2 a_SizePix;
in vec2 a_CoordPix;
in float a_Depth;

out vec2 v_TexCoord;

void main() {

    vec2 normalised_pos = vec2((a_Pos.x + 1.0) / 2.0,
                               (1.0 - a_Pos.y) / 2.0);

    vec2 coord_pix = a_CoordPix + a_SizePix * normalised_pos;
    vec2 normalised_coord = coord_pix / u_OutputSizePix;
    vec2 coord = vec2(normalised_coord.x * 2.0 - 1.0,
                      1.0 - normalised_coord.y * 2.0);

    vec2 tex_base = vec2(u_SpriteSize.x * a_SpriteIndex, 0.0);
    v_TexCoord = tex_base + u_SpriteSize * normalised_pos;
    v_TexCoord.y = 1 - v_TexCoord.y;

    gl_Position = vec4(coord, a_Depth, 1.0);
}
