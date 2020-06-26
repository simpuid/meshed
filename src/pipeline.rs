use crate::{Renderer, VertexTrait, IndexTrait};
use shaderc::{Compiler, ShaderKind};
use wgpu::*;
use std::io::Cursor;
use std::marker::PhantomData;
use std::any::TypeId;

pub trait PipelineTrait {
	fn get_render_pipeline(&self) -> &RenderPipeline;
}

pub struct Pipeline<V: VertexTrait, I: IndexTrait> {
	render_pipeline: RenderPipeline,
	vertex_marker: PhantomData<V>,
	index_marker: PhantomData<I>,
}

impl<V: VertexTrait, I: IndexTrait> Pipeline<V, I> {
	pub fn new(renderer: &Renderer, compiler: &mut Compiler, vertex_code: &str, fragment_code: &str, binds: &[TypeId]) -> Pipeline<V, I> {
		let vertex_mod = compile(renderer, compiler, vertex_code, ShaderKind::Vertex);
		let fragment_mod = compile(renderer, compiler, fragment_code, ShaderKind::Fragment);
		let pipeline_desc = RenderPipelineDescriptor {
			layout: renderer.get_pipeline_layout(binds).expect("pipeline_layout_not_registered"),
			vertex_stage: ProgrammableStageDescriptor {
				module: &vertex_mod,
				entry_point: "main",
			},
			fragment_stage: Some(ProgrammableStageDescriptor {
				module: &fragment_mod,
				entry_point: "main",
			}),
			rasterization_state: Some(RasterizationStateDescriptor {
				cull_mode: CullMode::Back,
				front_face: FrontFace::Ccw,
				depth_bias: 0,
				depth_bias_slope_scale: 0.0,
				depth_bias_clamp: 0.0,
			}),
			color_states: &[
				ColorStateDescriptor {
					format: TextureFormat::Bgra8UnormSrgb,
					color_blend: BlendDescriptor {
						src_factor: BlendFactor::SrcAlpha,
						dst_factor: BlendFactor::OneMinusSrcAlpha,
						operation: BlendOperation::Add,
					},
					alpha_blend: BlendDescriptor {
						src_factor: BlendFactor::One,
						dst_factor: BlendFactor::Zero,
						operation: BlendOperation::Add,
					},
					write_mask: ColorWrite::ALL,
				}
			],
			primitive_topology: PrimitiveTopology::TriangleList,
			depth_stencil_state: None,
			vertex_state: VertexStateDescriptor {
				index_format: I::index_format(),
				vertex_buffers: &[V::descriptor()],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		};
		let render_pipeline = renderer.get_device().create_render_pipeline(&pipeline_desc);
		Pipeline {
			render_pipeline,
			vertex_marker: PhantomData,
			index_marker: PhantomData,
		}
	}
}

impl<V: VertexTrait, I: IndexTrait> PipelineTrait for Pipeline<V, I> {
	fn get_render_pipeline(&self) -> &RenderPipeline {
		&self.render_pipeline
	}
}

fn compile(renderer: &Renderer, compiler: &mut Compiler, code: &str, kind: ShaderKind) -> ShaderModule {
	let output = compiler.compile_into_spirv(code, kind, "", "main", None).expect("shader_compilation_failed");
	let data = read_spirv(Cursor::new(output.as_binary_u8())).expect("shader_compilation_failed");
	renderer.get_device().create_shader_module(data.as_slice())
}