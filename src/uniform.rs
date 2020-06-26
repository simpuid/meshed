use crate::{Bind, Renderer};
use bytemuck::{bytes_of, Pod};
use std::marker::PhantomData;
use std::mem::size_of_val;
use wgpu::*;

pub struct Uniform<T: Pod> {
	buffer: Buffer,
	bind_group: BindGroup,
	phantom: PhantomData<T>,
}

impl<T: Pod> Uniform<T> {
	pub fn new(renderer: &Renderer, data: &T) -> Uniform<T> {
		let buffer = renderer
			.get_device()
			.create_buffer_with_data(bytes_of(data), BufferUsage::UNIFORM | BufferUsage::COPY_DST);
		let binding = Binding {
			binding: 0,
			resource: wgpu::BindingResource::Buffer {
				buffer: &buffer,
				range: 0..size_of_val(&data) as wgpu::BufferAddress,
			},
		};
		let bind_group_desc = BindGroupDescriptor {
			layout: renderer.get_bind_group_layout::<Self>().expect("uniform_type_not_registered"),
			bindings: &[binding],
			label: None,
		};
		let bind_group = renderer.get_device().create_bind_group(&bind_group_desc);

		Uniform {
			buffer,
			bind_group,
			phantom: PhantomData,
		}
	}

	pub fn update(&self, renderer: &Renderer, data: &T) {
		let data = bytes_of(data);
		let encoder_desc = CommandEncoderDescriptor {
			label: Some("uniform_update_encoder"),
		};
		let mut encoder = renderer.get_device().create_command_encoder(&encoder_desc);
		let staging_buffer = renderer
			.get_device()
			.create_buffer_with_data(data, BufferUsage::COPY_SRC);
		encoder.copy_buffer_to_buffer(&staging_buffer, 0, &self.buffer, 0, data.len() as u64);
		renderer.get_queue().submit(&[encoder.finish()]);
	}
}

impl<T: Pod> Bind for Uniform<T> {
	fn get_bind_group(&self) -> &BindGroup {
		&self.bind_group
	}

	fn get_bind_group_layout(device: &Device) -> BindGroupLayout {
		let layout_desc = BindGroupLayoutDescriptor {
			bindings: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStage::VERTEX,
				ty: wgpu::BindingType::UniformBuffer { dynamic: false },
			}],
			label: None,
		};
		device.create_bind_group_layout(&layout_desc)
	}
}
