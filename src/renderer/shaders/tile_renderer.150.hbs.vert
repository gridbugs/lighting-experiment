#version 150 core

{{INCLUDE_COMMON}}

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

uniform samplerBuffer t_VisionTable;

in vec2 a_Pos;

in vec2 a_SpriteSheetPixCoord;
in vec2 a_Position;
in vec2 a_PixSize;
in vec2 a_PixOffset;
in float a_Depth;
in uint a_DepthType;
in uint a_Flags;
in uint a_SpriteEffect;
in vec4 a_SpriteEffectArgs;
in uint a_HideInDark;

out vec2 v_TexCoord;
out float v_ColourMult;
flat out uint v_CellIndex;
out vec2 v_FragPosition;

const uint FLAGS_ENABLED = {{FLAGS_ENABLED}}u;
const uint FLAGS_SPRITE_EFFECT = {{FLAGS_SPRITE_EFFECT}}u;

const uint DEPTH_FIXED = {{DEPTH_FIXED}}u;
const uint DEPTH_GRADIENT = {{DEPTH_GRADIENT}}u;
const uint DEPTH_BOTTOM = {{DEPTH_BOTTOM}}u;

const uint SPRITE_EFFECT_WATER = {{SPRITE_EFFECT_WATER}}u;

uint cell_index() {
    vec2 pos = a_Position + vec2(0.5);
    return uint(pos.x) + uint(pos.y) * u_WorldSizeUint.x;
}

float u64_uvec2_to_float(uvec2 u) {
    const float MAXUINT_FLOAT = 4294967296.0;
    return float(u[1]) * MAXUINT_FLOAT + float(u[0]);
}

float water_colour_mult(float steps, float base_mult, float max_mult) {
    float x_orig = a_Position.x;
    float y_orig = a_Position.y;
    float t_orig = u64_uvec2_to_float(u_TotalTimeMs_u64);

    float x = x_orig * 10.0;
    float y = y_orig * 10.0;
    float t = t_orig / 160.0;

    const float PARTS = 4;
    float total =
        sin((x + sin(t / 23) * 11) / 7 + t / 11) +
        sin((x - sin(t / 17) * 13) / 5 + t / 29) +
        sin((y + sin(t / 11) * 17) / 11 + t / 17) +
        sin((y - sin(t / 19) * 23) / 13 + t / 23);

    float val = ((total / PARTS) + 1.0) / 2.0;

    float stepped = floor(val * steps) / steps;

    return base_mult + stepped * (max_mult - base_mult);
}

void main() {
    if ((a_Flags & FLAGS_ENABLED) == 0u) {
        gl_Position = vec4(0.0, 0.0, 0.0, -1.0);
        return;
    }

    v_ColourMult = 1.0;

    if ((a_Flags & FLAGS_SPRITE_EFFECT) != 0u) {
        switch (a_SpriteEffect) {
            case SPRITE_EFFECT_WATER:
                v_ColourMult *= water_colour_mult(a_SpriteEffectArgs[0], a_SpriteEffectArgs[1], a_SpriteEffectArgs[2]);
                break;
        }
    }

    v_CellIndex = cell_index();
    int vision_base = int(v_CellIndex * TBO_VISION_ENTRY_SIZE);

    uvec2 vision_timestamp = get_vision_timestamp(vision_base, t_VisionTable);

    if (!timestamp_is_seen(vision_timestamp)) {
        // if a cell has never been seen, don't draw it
        gl_Position = vec4(0.0, 0.0, 0.0, -1.0);
        return;
    }

    if (!timestamp_is_visible(vision_timestamp) && a_HideInDark == 1u) {
        // if an instance is hidden when not seen, don't draw it
        gl_Position = vec4(0.0, 0.0, 0.0, -1.0);
        return;
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

    v_FragPosition = a_Position + a_Pos;

    gl_Position = vec4(dst, depth, 1.0);
}
