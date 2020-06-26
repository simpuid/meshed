use winit::{
	window::{Window, WindowBuilder},
	event_loop::{EventLoop, ControlFlow},
	event::{Event, WindowEvent},
};
use meshed::{Renderer, Pipeline, Command, Vertex, Mesh, Texture, Uniform};
use shaderc::Compiler;
use cgmath::{Matrix4, Deg};
use std::any::TypeId;
use winit::dpi::{Size, PhysicalSize};
use bytemuck::{Pod, Zeroable};
use std::time::Instant;

fn main() {
	let event_loop = EventLoop::new();
	let mut game = Game::new(&event_loop);
	event_loop.run(move |event, _, control_flow| {
		*control_flow = game.handle_event(event);
	});
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Transform {
	matrix: Matrix4<f32>
}

unsafe impl Pod for Transform {}

unsafe impl Zeroable for Transform {}

struct Game {
	window: Window,
	renderer: Renderer,
	pipeline: Pipeline<Vertex, u16>,
	mesh: Mesh<Vertex, u16>,
	texture: Texture,
	uniform: Uniform<Transform>,
	instant: Instant,
}

impl Game {
	fn new(event_loop: &EventLoop<()>) -> Game {
		let window = WindowBuilder::new().with_inner_size(Size::Physical(PhysicalSize::new(512, 512))).build(&event_loop).unwrap();
		let mut renderer = Renderer::new(&window, window.inner_size().width, window.inner_size().height);
		let mut compiler = Compiler::new().unwrap();
		renderer.register_bind_group_layout::<Texture>();
		renderer.register_bind_group_layout::<Uniform<Transform>>();
		renderer.register_pipeline_layout(&[TypeId::of::<Texture>(), TypeId::of::<Uniform<Transform>>()]);
		let pipeline = Pipeline::new(&mut renderer, &mut compiler, include_str!("vertex.glsl"), include_str!("fragment.glsl"), &[TypeId::of::<Texture>(), TypeId::of::<Uniform<Transform>>()]);
		let mesh = Mesh::new(&renderer, &[
			Vertex::new([-0.5, -0.5, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0]),
			Vertex::new([0.5, -0.5, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0]),
			Vertex::new([0.5, 0.5, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0]),
			Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0]),
		], &[0, 1, 2, 0, 2, 3]);
		let texture = Texture::new(&renderer, include_bytes!("logo.png"));
		let matrix = Transform {
			matrix: Matrix4::from_angle_z(Deg(1.0))
		};
		let uniform = Uniform::new(&renderer, &matrix);

		Game {
			window,
			renderer,
			pipeline,
			mesh,
			texture,
			uniform,
			instant: Instant::now(),
		}
	}

	fn handle_event(&mut self, event: Event<()>) -> ControlFlow {
		match event {
			Event::WindowEvent { window_id, event } => {
				if window_id == self.window.id() {
					match event {
						WindowEvent::CloseRequested => {
							return ControlFlow::Exit;
						}
						WindowEvent::Resized(size) => {
							self.renderer.resize(size.width, size.height);
						}
						WindowEvent::ScaleFactorChanged { scale_factor: _, new_inner_size } => {
							self.renderer.resize(new_inner_size.width, new_inner_size.height);
						}
						_ => ()
					}
				}
			}
			Event::RedrawRequested(window_id) => {
				let transform = Transform {
					matrix: Matrix4::from_angle_z(Deg(ease_in_out_cubic((self.instant.elapsed().as_secs_f32() * 0.5) % 1.0) * -360.0))
				};
				self.uniform.update(&self.renderer, &transform);
				if window_id == self.window.id() {
					let commands = [
						Command::SetPipeline(&self.pipeline),
						Command::SetBind(&self.texture, 0),
						Command::SetBind(&self.uniform, 1),
						Command::SetMesh(&self.mesh),
						Command::Draw(0..6),
					];
					Command::execute(&mut self.renderer, &[
						color_cos(self.instant.elapsed().as_secs_f64(), 0.5),
						color_cos(self.instant.elapsed().as_secs_f64(), 1.0),
						color_cos(self.instant.elapsed().as_secs_f64(), 2.0),
						1.0
					], &commands);
				}
			}
			Event::MainEventsCleared => {
				self.window.request_redraw();
			}
			_ => ()
		}
		ControlFlow::Poll
	}
}

fn ease_in_out_cubic(x: f32) -> f32 {
	if x < 0.5 {
		4.0 * x * x * x
	} else {
		let inter = -2.0 * x + 2.0;
		1.0 - inter * inter * inter / 2.0
	}
}

fn color_cos(t: f64, factor: f64) -> f64 {
	0.5 + f64::cos(t * factor * std::f64::consts::PI) * 0.5
}