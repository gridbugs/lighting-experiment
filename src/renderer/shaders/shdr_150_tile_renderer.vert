#version 150 core

uniform Dimensions {
    vec2 u_SpriteSheetSize;
    vec2 u_OutputSize;
};

uniform Offset {
    vec2 u_ScrollOffsetPix;
};

in vec2 a_Pos;

in vec2 a_SpriteSheetPixCoord;
in vec2 a_OutPixCoord;
in vec2 a_PixSize;
in vec2 a_PixOffset;
in float a_Depth;

out vec2 v_TexCoord;

void main() {

    vec2 in_pix = a_SpriteSheetPixCoord + a_Pos * a_PixSize;
    v_TexCoord = in_pix / u_SpriteSheetSize;
    v_TexCoord.y = 1.0 - v_TexCoord.y;

    vec2 out_pix = a_OutPixCoord - u_ScrollOffsetPix - a_PixOffset + a_Pos * a_PixSize;
    vec2 out_scaled = out_pix / u_OutputSize;
    vec2 dst = vec2(out_scaled.x * 2.0 - 1.0, 1.0 - out_scaled.y * 2.0);
    gl_Position = vec4(dst, a_Depth, 1.0);
}
