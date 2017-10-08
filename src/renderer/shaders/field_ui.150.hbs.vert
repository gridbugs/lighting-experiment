#version 150 core

{{INCLUDE_VISION}}
{{INCLUDE_DIMENSIONS}}
{{INCLUDE_SCROLL_OFFSET}}
{{INCLUDE_POSITIONS}}

uniform samplerBuffer t_VisionTable;

in vec2 a_Pos;
out vec2 v_TexCoord;

in vec2 a_SpriteSheetPixCoord;
in vec2 a_Position;
in vec2 a_PixSize;
in vec2 a_PixOffset;
in float a_Depth;

void main() {
    uint cell_index = get_cell_index(a_Position);
    int vision_base = int(cell_index * TBO_VISION_ENTRY_SIZE);
    uvec2 vision_timestamp = get_vision_timestamp(vision_base, t_VisionTable);
    if (!timestamp_is_visible(vision_timestamp)) {
        gl_Position = vec4(0.0, 0.0, -1.0, 0.0);
        return;
    }

    v_TexCoord = get_tex_coord_inverted(a_SpriteSheetPixCoord, a_Pos, a_PixSize);
    vec2 dst = get_output_vertex(a_Position, a_PixOffset, a_PixSize, a_Pos);
    gl_Position = vec4(dst, a_Depth, 1.0);
}
