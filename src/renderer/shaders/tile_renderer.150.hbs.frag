#version 150 core

{{INCLUDE_COMMON}}

struct Light {
    vec3 colour;
    vec3 position;
    float intensity;
};

uniform LightList {
    Light u_Lights[MAX_NUM_LIGHTS];
};

uniform samplerBuffer t_LightTable;
uniform samplerBuffer t_VisionTable;

uniform sampler2D t_Texture;

in vec2 v_FragPosition;
in vec2 v_TexCoord;
in float v_ColourMult;
flat in uint v_CellIndex;

out vec4 Target0;

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
