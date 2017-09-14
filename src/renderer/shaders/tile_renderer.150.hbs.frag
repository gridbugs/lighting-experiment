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

const uint TBO_VISION_BITMAP_OFFSET = {{TBO_VISION_BITMAP_OFFSET}}u;
const uint TBO_VISION_ENTRY_SIZE = {{TBO_VISION_ENTRY_SIZE}}u;
const uint TBO_VISION_BUFFER_SIZE = {{TBO_VISION_BUFFER_SIZE}}u;
uniform samplerBuffer t_LightTable;
uniform samplerBuffer t_VisionTable;

uniform sampler2D t_Texture;

in vec2 v_FragPosition;
in vec2 v_TexCoord;
in float v_ColourMult;
flat in uint v_CellIndex;

out vec4 Target0;

uvec2 get_vision_timestamp(int base, samplerBuffer table) {
    uint lo = uint(texelFetch(table, base).r * 255) +
        (uint(texelFetch(table, base + 1).r * 255) << 8) +
        (uint(texelFetch(table, base + 2).r * 255) << 16) +
        (uint(texelFetch(table, base + 3).r * 255) << 24);

    uint hi = uint(texelFetch(table, base + 4).r * 255);

    return uvec2(lo, hi);
}

uint get_vision_bitmap(int base, samplerBuffer table) {
    return uint(texelFetch(table, base + int(TBO_VISION_BITMAP_OFFSET)).r * 255);
}

bool timestamp_is_visible(uvec2 timestamp) {
    return timestamp == u_FrameCount_u64;
}

uint get_lit_sides(uint i) {
    int base = int(i * TBO_VISION_BUFFER_SIZE + v_CellIndex * TBO_VISION_ENTRY_SIZE);
    if (timestamp_is_visible(get_vision_timestamp(base, t_LightTable))) {
        return get_vision_bitmap(base, t_LightTable);
    }
    return 0u;
}

const vec3 VERTICAL = vec3(0, 0, 1);

vec3 diffuse_light(Light light, vec3 surface_colour) {
    vec3 direction = normalize(light.position - vec3(v_FragPosition, 0));
    return surface_colour * light.colour * light.intensity * dot(direction, VERTICAL);
}

const float AMBIENT_LIGHT_MULT = 0.1;

void main() {

    vec4 tex_colour = texture(t_Texture, v_TexCoord);
    if (tex_colour.a < 0.001) {
        discard;
    }

    vec3 base_colour = tex_colour.rgb * v_ColourMult;

    int vision_base = int(v_CellIndex * TBO_VISION_ENTRY_SIZE);
    uint vision_bitmap = get_vision_bitmap(vision_base, t_VisionTable);
    uvec2 vision_timestamp = get_vision_timestamp(vision_base, t_VisionTable);

    uint side_bitmap;

    vec3 diffuse_total = vec3(0);
    if (timestamp_is_visible(vision_timestamp)) {
        for (uint i = 0u; i < u_NumLights; i++) {
            uint lit_sides = get_lit_sides(i);
            uint visible_lit_sides = lit_sides & vision_bitmap;
            if (visible_lit_sides != 0u) {
                diffuse_total += diffuse_light(u_Lights[i], base_colour);
            }
        }
    }

    vec3 ambient_total = base_colour * AMBIENT_LIGHT_MULT;

    Target0 = vec4((ambient_total + diffuse_total), 1);
}
