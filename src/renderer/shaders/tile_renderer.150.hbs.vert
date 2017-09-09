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

uniform FrameInfo {
    uvec2 u_Time_u64;
};

struct Cell {
    uvec2 last_seen_u64;
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
in uint a_Flags;

out vec2 v_TexCoord;
out float v_ColourMult;

const uint FLAGS_ENABLED = {{FLAGS_ENABLED}}u;

const uint DEPTH_FIXED = {{DEPTH_FIXED}}u;
const uint DEPTH_GRADIENT = {{DEPTH_GRADIENT}}u;
const uint DEPTH_BOTTOM = {{DEPTH_BOTTOM}}u;

Cell get_cell() {
    vec2 pos = a_Position + vec2(0.5);
    int idx = int(pos.x) + int(pos.y) * int(u_WorldSizeUint.x);
    return u_Cells[idx];
}

bool cell_is_seen(Cell cell) {
    return cell.last_seen_u64 != uvec2(0, 0);
}

bool cell_is_visible(Cell cell) {
    return cell.last_seen_u64 == u_Time_u64;
}

void main() {

    if ((a_Flags & FLAGS_ENABLED) == 0u) {
        gl_Position = vec4(0.0, 0.0, 0.0, -1.0);
        return;
    }

    Cell cell = get_cell();

    if (!cell_is_seen(cell)) {
        gl_Position = vec4(0.0, 0.0, 0.0, -1.0);
        return;
    }

    if (cell_is_visible(cell)) {
        v_ColourMult = 1.0;
    } else {
        v_ColourMult = 0.05;
    }

    float depth = -1;
    switch (a_DepthType) {
        case DEPTH_FIXED:
            depth = 1.0 - a_Depth / u_WorldSize.y;
            break;
        case DEPTH_GRADIENT:
            depth = 1.0 - (a_Depth - 1.0 + a_Pos[1]) / u_WorldSize.y;
            break;
        case DEPTH_BOTTOM:
            depth = 1.0 - a_Depth;
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
