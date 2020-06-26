use wgpu::*;
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

pub trait VertexTrait: Pod {
	fn descriptor<'a>() -> VertexBufferDescriptor<'a>;
}

pub trait IndexTrait: Pod {
	fn index_format() -> IndexFormat;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
	uv: [f32; 2],
}

impl Vertex {
	pub fn new(position: [f32; 3], color: [f32; 3], uv: [f32; 2]) -> Vertex {
		Vertex {
			position,
			color,
			uv,
		}
	}
}

unsafe impl Pod for Vertex {}

unsafe impl Zeroable for Vertex {}

impl VertexTrait for Vertex {
	fn descriptor<'a>() -> VertexBufferDescriptor<'a> {
		VertexBufferDescriptor {
			stride: size_of::<Vertex>() as BufferAddress,
			step_mode: InputStepMode::Vertex,
			attributes: &[
				VertexAttributeDescriptor {
					offset: 0,
					shader_location: 0,
					format: VertexFormat::Float3,
				},
				VertexAttributeDescriptor {
					offset: size_of::<[f32; 3]>() as BufferAddress,
					shader_location: 1,
					format: VertexFormat::Float3,
				},
				VertexAttributeDescriptor {
					offset: (size_of::<[f32; 3]>() * 2) as BufferAddress,
					shader_location: 2,
					format: VertexFormat::Float2,
				},
			],
		}
	}
}

impl IndexTrait for u16 {
	fn index_format() -> IndexFormat {
		IndexFormat::Uint16
	}
}

impl IndexTrait for u32 {
	fn index_format() -> IndexFormat {
		IndexFormat::Uint32
	}
}