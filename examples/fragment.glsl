#version 450

layout(location=0) in vec3 v_color;
layout(location=1) in vec2 v_uv;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D f_texture;
layout(set = 0, binding = 1) uniform sampler f_sampler;

void main() {
	f_color = texture(sampler2D(f_texture, f_sampler), v_uv);
}