use crate::{Renderer, Bind};
use image::GenericImageView;
use wgpu::*;

pub struct Texture {
	_texture: wgpu::Texture,
	_view: TextureView,
	_sampler: Sampler,
	bind_group: BindGroup,
}

impl Texture {
	pub fn new(renderer: &Renderer, data: &[u8]) -> Texture {
		let image = image::load_from_memory(data).expect("image_reading_failed").flipv();
		let (width, height) = image.dimensions();
		let rgba = image.as_rgba8().expect("image_conversion_failed").clone().into_raw();
		let size = wgpu::Extent3d {
			width,
			height,
			depth: 1,
		};
		let _texture = renderer.get_device().create_texture(&wgpu::TextureDescriptor {
			size,
			array_layer_count: 1,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
			label: None,
		});
		let buffer = renderer.get_device().create_buffer_with_data(rgba.as_slice(), BufferUsage::COPY_SRC);
		let encoder_desc = CommandEncoderDescriptor {
			label: Some("texture_data_copy_encoder")
		};
		let mut encoder = renderer.get_device().create_command_encoder(&encoder_desc);
		let buffer_copy_view = BufferCopyView {
			buffer: &buffer,
			offset: 0,
			bytes_per_row: 4 * width,
			rows_per_image: height,
		};
		let texture_copy_view = TextureCopyView {
			texture: &_texture,
			mip_level: 0,
			array_layer: 0,
			origin: Origin3d::ZERO,
		};
		let copy_size = Extent3d {
			width,
			height,
			depth: 1,
		};
		encoder.copy_buffer_to_texture(buffer_copy_view, texture_copy_view, copy_size);
		renderer.get_queue().submit(&[encoder.finish()]);
		let sampler_desc = SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::FilterMode::Nearest,
			lod_min_clamp: -100.0,
			lod_max_clamp: 100.0,
			compare: wgpu::CompareFunction::Always,
		};
		let _sampler = renderer.get_device().create_sampler(&sampler_desc);
		let _view = _texture.create_default_view();
		let bind_group_desc = BindGroupDescriptor {
			layout: renderer.get_bind_group_layout::<Self>().expect("texture_type_not_registered"),
			bindings: &[
				wgpu::Binding {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&_view),
				},
				wgpu::Binding {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&_sampler),
				}
			],
			label: None,
		};
		let bind_group = renderer.get_device().create_bind_group(&bind_group_desc);

		Texture {
			_texture,
			_view,
			_sampler,
			bind_group,
		}
	}

	pub fn get_bind_group(&self) -> &BindGroup {
		&self.bind_group
	}
}

impl Bind for Texture {
	fn get_bind_group(&self) -> &BindGroup {
		&self.bind_group
	}

	fn get_bind_group_layout(device: &Device) -> BindGroupLayout {
		let layout = BindGroupLayoutDescriptor {
			bindings: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStage::FRAGMENT,
					ty: wgpu::BindingType::SampledTexture {
						multisampled: false,
						dimension: wgpu::TextureViewDimension::D2,
						component_type: wgpu::TextureComponentType::Uint,
					},
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStage::FRAGMENT,
					ty: wgpu::BindingType::Sampler {
						comparison: false,
					},
				},
			],
			label: None,
		};
		device.create_bind_group_layout(&layout)
	}
}