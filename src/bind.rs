use wgpu::*;

pub trait Bind {
	fn get_bind_group(&self) -> &BindGroup;
	fn get_bind_group_layout(device: &Device) -> BindGroupLayout where Self: Sized;
}