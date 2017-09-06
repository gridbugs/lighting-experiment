#version 150 core

uniform FixedDimensions {
    vec2 u_SpriteSheetSize;
    vec2 u_CellSize;
};

uniform OutputDimensions {
    vec2 u_OutputSize;
};

uniform WorldDimensions {
    vec2 u_WorldSize;
    uvec2 u_WorldSizeUint;
};

uniform Offset {
    vec2 u_ScrollOffsetPix;
};

struct Cell {
    uint flags;
};

const uint MAX_CELL_TABLE_SIZE = {{MAX_CELL_TABLE_SIZE}}u;
uniform CellTable {
    Cell u_Cells[MAX_CELL_TABLE_SIZE];
};

in vec2 a_Pos;

in vec2 a_SpriteSheetPixCoord;
in vec2 a_Position;
in vec2 a_PixSize;
in vec2 a_PixOffset;
in float a_Depth;
in uint a_DepthType;

out vec2 v_TexCoord;

const uint DEPTH_DISABLED = {{DEPTH_DISABLED}}u;
const uint DEPTH_FIXED = {{DEPTH_FIXED}}u;
const uint DEPTH_GRADIENT = {{DEPTH_GRADIENT}}u;
const uint DEPTH_BOTTOM = {{DEPTH_BOTTOM}}u;

const uint CELL_VISIBLE = {{CELL_VISIBLE}}u;

void main() {

    float depth = -1;
    switch (a_DepthType) {
        case DEPTH_DISABLED:
            gl_Position = vec4(0.0, 0.0, 0.0, -1.0);
            return;
        case DEPTH_FIXED:
            depth = 1.0 - a_Depth / u_WorldSize.y;
            break;
        case DEPTH_GRADIENT:
            depth = 1.0 - (a_Depth - 1.0 + a_Pos[1]) / u_WorldSize.y;
            break;
        case DEPTH_BOTTOM:
            depth = 1.0;
            break;
    }

    vec2 in_pix = a_SpriteSheetPixCoord + a_Pos * a_PixSize;
    v_TexCoord = in_pix / u_SpriteSheetSize;
    v_TexCoord.y = 1.0 - v_TexCoord.y;

    vec2 out_pix = a_Position * u_CellSize - u_ScrollOffsetPix - a_PixOffset + a_Pos * a_PixSize;
    vec2 out_scaled = out_pix / u_OutputSize;
    vec2 dst = vec2(out_scaled.x * 2.0 - 1.0, 1.0 - out_scaled.y * 2.0);

    gl_Position = vec4(dst, depth, 1.0);
}
