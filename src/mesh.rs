use crate::{VertexTrait, Renderer, IndexTrait};
use wgpu::*;
use bytemuck::cast_slice;
use std::marker::PhantomData;

pub trait MeshTrait {
	fn get_vertex_buffer(&self) -> &Buffer;
	fn get_index_buffer(&self) -> &Buffer;
}

pub struct Mesh<V: VertexTrait, I: IndexTrait> {
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	vertex_buffer_size: usize,
	index_buffer_size: usize,
	vertex_marker: PhantomData<V>,
	index_marker: PhantomData<I>,
}

impl<V: VertexTrait, I: IndexTrait> Mesh<V, I> {
	pub fn new(renderer: &Renderer, vertices: &[V], indices: &[I]) -> Mesh<V, I> {
		let vertex_data = cast_slice(vertices);
		let index_data = cast_slice(indices);
		let vertex_buffer = renderer.get_device().create_buffer_with_data(vertex_data, BufferUsage::VERTEX | BufferUsage::COPY_DST);
		let index_buffer = renderer.get_device().create_buffer_with_data(index_data, BufferUsage::INDEX | BufferUsage::COPY_DST);

		Mesh {
			vertex_buffer,
			index_buffer,
			vertex_buffer_size: vertex_data.len(),
			index_buffer_size: index_data.len(),
			vertex_marker: PhantomData,
			index_marker: PhantomData,
		}
	}

	pub fn update_vertex(&mut self, renderer: &Renderer, vertices: &[V]) {
		let data = cast_slice(vertices);
		if data.len() <= self.vertex_buffer_size {
			copy_data_to_buffer(renderer, &self.vertex_buffer, data);
		} else {
			self.vertex_buffer_size = data.len();
			self.vertex_buffer = renderer.get_device().create_buffer_with_data(data, BufferUsage::VERTEX | BufferUsage::COPY_DST);
		}
	}

	pub fn update_index(&mut self, renderer: &Renderer, indices: &[I]) {
		let data = cast_slice(indices);
		if data.len() <= self.index_buffer_size {
			copy_data_to_buffer(renderer, &self.index_buffer, data);
		} else {
			self.index_buffer_size = data.len();
			self.index_buffer = renderer.get_device().create_buffer_with_data(data, BufferUsage::INDEX | BufferUsage::COPY_DST);
		}
	}

	pub fn update_vertex_packed(&mut self, renderer: &Renderer, vertices: &[V]) {
		let data = cast_slice(vertices);
		if data.len() == self.vertex_buffer_size {
			copy_data_to_buffer(renderer, &self.index_buffer, data);
		} else {
			self.vertex_buffer_size = data.len();
			self.vertex_buffer = renderer.get_device().create_buffer_with_data(data, BufferUsage::VERTEX | BufferUsage::COPY_DST);
		}
	}

	pub fn update_index_packed(&mut self, renderer: &Renderer, indices: &[I]) {
		let data = cast_slice(indices);
		if data.len() == self.index_buffer_size {
			copy_data_to_buffer(renderer, &self.index_buffer, data);
		} else {
			self.index_buffer_size = data.len();
			self.index_buffer = renderer.get_device().create_buffer_with_data(data, BufferUsage::INDEX | BufferUsage::COPY_DST);
		}
	}
}

impl<V: VertexTrait, I: IndexTrait> MeshTrait for Mesh<V, I> {
	fn get_vertex_buffer(&self) -> &Buffer {
		&self.vertex_buffer
	}

	fn get_index_buffer(&self) -> &Buffer {
		&self.index_buffer
	}
}

fn copy_data_to_buffer(renderer: &Renderer, buffer: &Buffer, data: &[u8]) {
	let encoder_desc = CommandEncoderDescriptor {
		label: Some("mesh_data_copy_encoder")
	};
	let mut encoder = renderer.get_device().create_command_encoder(&encoder_desc);
	let staging_buffer = renderer.get_device().create_buffer_with_data(data, BufferUsage::COPY_SRC);
	encoder.copy_buffer_to_buffer(&staging_buffer, 0, buffer, 0, data.len() as u64);
	renderer.get_queue().submit(&[encoder.finish()]);
}