#version 150 core

const uint MAX_NUM_LIGHTS = {{MAX_NUM_LIGHTS}}u;

uniform FrameInfo {
    uvec2 u_FrameCount_u64;
    uvec2 u_TotalTimeMs_u64;
    uint u_NumLights;
};

struct Light {
    vec3 colour;
    vec3 position;
    float intensity;
};

uniform LightList {
    Light u_Lights[MAX_NUM_LIGHTS];
};

const uint LIGHT_BUFFER_OFFSET_SIDE_BITMAP = {{LIGHT_BUFFER_OFFSET_SIDE_BITMAP}}u;
const uint LIGHT_BUFFER_ENTRY_SIZE = {{LIGHT_BUFFER_ENTRY_SIZE}}u;
const uint LIGHT_BUFFER_SIZE_PER_LIGHT = {{LIGHT_BUFFER_SIZE_PER_LIGHT}}u;
uniform samplerBuffer t_LightTable;

struct Cell {
    uvec2 last_u64;
    uint side_bitmap;
};

const uint MAX_CELL_TABLE_SIZE = {{MAX_CELL_TABLE_SIZE}}u;
uniform VisionTable {
    Cell u_VisionCells[MAX_CELL_TABLE_SIZE];
};

uniform sampler2D t_Texture;

in vec2 v_TexCoord;
in float v_ColourMult;
flat in uint v_CellIndex;

out vec4 Target0;

uvec2 light_timestamp(int base) {
    uint lo = uint(texelFetch(t_LightTable, base).r * 255) +
        (uint(texelFetch(t_LightTable, base + 1).r * 255) << 8) +
        (uint(texelFetch(t_LightTable, base + 2).r * 255) << 16) +
        (uint(texelFetch(t_LightTable, base + 3).r * 255) << 24);

    uint hi = uint(texelFetch(t_LightTable, base + 4).r * 255);

    return uvec2(lo, hi);
}

uint get_lit_sides(uint i) {
    int base = int(i * LIGHT_BUFFER_SIZE_PER_LIGHT + v_CellIndex * LIGHT_BUFFER_ENTRY_SIZE);
    if (light_timestamp(base) == u_FrameCount_u64) {
        return uint(texelFetch(t_LightTable, base + int(LIGHT_BUFFER_OFFSET_SIDE_BITMAP)).r * 255);
    }
    return 0u;
}

void main() {

    vec4 tex_colour = texture(t_Texture, v_TexCoord);
    if (tex_colour.a < 0.001) {
        discard;
    }

    Cell vision_cell = u_VisionCells[v_CellIndex];

    for (uint i = 0u; i < u_NumLights; i++) {
        uint lit_sides = get_lit_sides(i);
        uint visible_lit_sides = lit_sides & vision_cell.side_bitmap;
        if (visible_lit_sides != 0u) {
            Light light = u_Lights[i];
            // TODO lighting goes here
        }
    }

    tex_colour.r *= v_ColourMult;
    tex_colour.g *= v_ColourMult;
    tex_colour.b *= v_ColourMult;
    Target0 = tex_colour;
}
