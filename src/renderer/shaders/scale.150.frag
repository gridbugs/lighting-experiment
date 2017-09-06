#version 150 core

uniform sampler2D t_Texture;

uniform Info {
    vec2 u_TexSize;
    float u_InterpolateThresholdFromCentre;
    float u_InterpolateStripWidth;
};

in vec2 v_Texel;

out vec4 Target0;

vec4 sample_texture(vec2 texel_coord) {
    return texture(t_Texture, texel_coord / u_TexSize);
}

void main() {

    vec2 sample_point = v_Texel + vec2(0.001); // XXX: Why is this offset necessary?

    // Vector from centre of texel containing sampling point to
    // the sampling point itself.
    vec2 from_texel_centre = fract(sample_point) - vec2(0.5);

    // Colour of texel containing sampling point. Note that this
    // assumes the sampler uses nearest-neighbour interpolation.
    vec4 main_col = sample_texture(sample_point);

    vec4 x_col = main_col;
    if (from_texel_centre.x > u_InterpolateThresholdFromCentre) {
        // sample within interpolation strip on right side of texel
        vec2 next_texel_coord = sample_point + vec2(0.5, 0.0);
        if (next_texel_coord.x <= u_TexSize.x) {
            vec4 next_col = sample_texture(next_texel_coord);
            float weight = (from_texel_centre.x - u_InterpolateThresholdFromCentre)
                / u_InterpolateStripWidth;
            x_col = weight * main_col + (1.0 - weight) * next_col;
        }
    } else if (from_texel_centre.x < -u_InterpolateThresholdFromCentre) {
        // sample within interpolation strip on left side of texel
        vec2 next_texel_coord = sample_point - vec2(0.5, 0.0);
        if (next_texel_coord.x >= 0.0) {
            vec4 next_col = sample_texture(next_texel_coord);
            float weight = -(from_texel_centre.x + u_InterpolateThresholdFromCentre)
                / u_InterpolateStripWidth;
            x_col = weight * main_col + (1.0 - weight) * next_col;
        }
    }

    vec4 y_col = main_col;
    if (from_texel_centre.y > u_InterpolateThresholdFromCentre) {
        // sample within interpolation strip on bottom side of texel
        vec2 next_texel_coord = sample_point + vec2(0.0, 0.5);
        if (next_texel_coord.y <= u_TexSize.y) {
            vec4 next_col = sample_texture(next_texel_coord);
            float weight = (from_texel_centre.y - u_InterpolateThresholdFromCentre)
                / u_InterpolateStripWidth;
            y_col = weight * main_col + (1.0 - weight) * next_col;
        }
    } else if (from_texel_centre.y < -u_InterpolateThresholdFromCentre) {
        // sample within interpolation strip on top side of texel
        vec2 next_texel_coord = sample_point - vec2(0.0, 0.5);
        if (next_texel_coord.y >= 0.0) {
            vec4 next_col = sample_texture(next_texel_coord);
            float weight = -(from_texel_centre.y + u_InterpolateThresholdFromCentre)
                / u_InterpolateStripWidth;
            y_col = weight * main_col + (1.0 - weight) * next_col;
        }
    }

    Target0 = (x_col + y_col) / 2.0;
}
