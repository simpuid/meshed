#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;
layout(location=2) in vec2 a_uv;

layout(location=0) out vec3 v_color;
layout(location=1) out vec2 v_uv;

layout(set=1, binding=0)
uniform Uniforms {
	mat4 matrix;
};

void main() {
	v_color = a_color;
	v_uv = a_uv;
	gl_Position = matrix * vec4(a_position, 1.0);
}