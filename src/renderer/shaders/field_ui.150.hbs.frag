#version 150 core

out vec4 Target0;

in vec2 v_TexCoord;
uniform sampler2D t_Texture;

void main() {
    Target0 = texture(t_Texture, v_TexCoord);
}
