#version 330 core

uniform sampler2D t_Texture;

in vec2 v_TexCoord;

out vec4 Target0;

void main() {
    Target0 = texture(t_Texture, v_TexCoord);
}
