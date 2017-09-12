#version 150 core

const uint MAX_NUM_LIGHTS = {{MAX_NUM_LIGHTS}}u;

struct Light {
    vec3 colour;
    vec3 position;
    float intensity;
};

uniform LightList {
    Light u_Lights[MAX_NUM_LIGHTS];
};

uniform sampler2D t_Texture;

in vec2 v_TexCoord;
in float v_ColourMult;

out vec4 Target0;

void main() {
    vec4 colour = texture(t_Texture, v_TexCoord);
    if (colour.a < 0.001) {
        discard;
    }

    colour.r *= v_ColourMult;
    colour.g *= v_ColourMult;
    colour.b *= v_ColourMult;
    Target0 = colour;
}
