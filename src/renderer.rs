use crate::Bind;
use raw_window_handle::HasRawWindowHandle;
use wgpu::*;
use futures::executor::block_on;
use std::collections::HashMap;
use std::any::TypeId;
use std::collections::hash_map::Entry;

pub struct Renderer {
	surface: Surface,
	device: Device,
	queue: Queue,
	swap_chain: SwapChain,
	swap_chain_desc: SwapChainDescriptor,
	bind_group_layout: HashMap<TypeId, BindGroupLayout>,
	pipeline_layouts: HashMap<Vec<TypeId>, PipelineLayout>,
}

impl Renderer {
	pub fn new<T: HasRawWindowHandle>(window: &T, width: u32, height: u32) -> Renderer {
		let surface = Surface::create(window);
		let adapter_option = RequestAdapterOptions {
			power_preference: PowerPreference::Default,
			compatible_surface: Some(&surface),
		};
		let adapter = block_on(Adapter::request(&adapter_option, BackendBit::PRIMARY)).expect("adaptor_creation_failed");
		let device_desc = DeviceDescriptor {
			extensions: Extensions {
				anisotropic_filtering: false
			},
			limits: Default::default(),
		};
		let (device, queue) = block_on(adapter.request_device(&device_desc));
		let swap_chain_desc = SwapChainDescriptor {
			usage: TextureUsage::OUTPUT_ATTACHMENT,
			format: TextureFormat::Bgra8UnormSrgb,
			present_mode: PresentMode::Fifo,
			width,
			height,
		};
		let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

		Renderer {
			surface,
			device,
			queue,
			swap_chain,
			swap_chain_desc,
			bind_group_layout: HashMap::new(),
			pipeline_layouts: HashMap::new(),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.swap_chain_desc.width = width;
		self.swap_chain_desc.height = height;
		self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);
	}

	pub fn get_device(&self) -> &Device {
		&self.device
	}

	pub fn get_device_mut(&mut self) -> &mut Device {
		&mut self.device
	}

	pub fn get_queue(&self) -> &Queue {
		&self.queue
	}

	pub fn get_queue_mut(&mut self) -> &mut Queue {
		&mut self.queue
	}

	pub fn get_swap_chain(&self) -> &SwapChain {
		&self.swap_chain
	}

	pub fn get_swap_chain_mut(&mut self) -> &mut SwapChain {
		&mut self.swap_chain
	}

	pub fn register_bind_group_layout<T: 'static + Bind>(&mut self) {
		let id = TypeId::of::<T>();
		match self.bind_group_layout.entry(id) {
			Entry::Vacant(v) => {
				v.insert(T::get_bind_group_layout(&self.device));
			}
			Entry::Occupied(_) => {
				panic!("bind_group_layout_already_registered");
			}
		}
	}

	pub fn get_bind_group_layout<T: 'static + Bind>(&self) -> Option<&BindGroupLayout> {
		self.bind_group_layout.get(&TypeId::of::<T>())
	}

	pub fn register_pipeline_layout(&mut self, binds: &[TypeId]) {
		match self.pipeline_layouts.entry(Vec::from(binds)) {
			Entry::Vacant(v) => {
				let mut bind_group_layout: Vec<&BindGroupLayout> = Vec::new();
				for id in binds {
					bind_group_layout.push(self.bind_group_layout.get(id).expect("type_not_registered"));
				}
				let pipeline_layout_desc = PipelineLayoutDescriptor {
					bind_group_layouts: bind_group_layout.as_slice()
				};
				v.insert(self.device.create_pipeline_layout(&pipeline_layout_desc));
			}
			Entry::Occupied(_) => {
				panic!("pipeline_layout_already_registered");
			}
		}
	}

	pub fn get_pipeline_layout(&self, binds: &[TypeId]) -> Option<&PipelineLayout> {
		self.pipeline_layouts.get(binds)
	}
}