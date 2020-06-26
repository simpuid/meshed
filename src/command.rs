use crate::{Renderer, Bind, PipelineTrait, MeshTrait};
use std::ops::Range;
use wgpu::*;

pub enum Command<'a> {
	Pass,
	SetPipeline(&'a dyn PipelineTrait),
	SetMesh(&'a dyn MeshTrait),
	SetBind(&'a dyn Bind, u32),
	Draw(Range<u32>),
}

impl<'a> Command<'a> {
	pub fn execute(renderer: &mut Renderer, clear_color: &[f64; 4], commands: &[Command]) {
		let frame = renderer.get_swap_chain_mut().get_next_texture().expect("texture_finding_failed");
		let encoder_desc = CommandEncoderDescriptor {
			label: Some("execute_encoder")
		};
		let mut encoder = renderer.get_device().create_command_encoder(&encoder_desc);
		{
			let render_pass_color_attachment_desc = RenderPassColorAttachmentDescriptor {
				attachment: &frame.view,
				resolve_target: None,
				load_op: LoadOp::Clear,
				store_op: StoreOp::Store,
				clear_color: Color { r: clear_color[0], g: clear_color[1], b: clear_color[2], a: clear_color[3] },
			};
			let render_pass_desc = RenderPassDescriptor {
				color_attachments: &[render_pass_color_attachment_desc],
				depth_stencil_attachment: None,
			};
			let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
			for command in commands {
				match command {
					Command::SetPipeline(pipeline) => {
						render_pass.set_pipeline(pipeline.get_render_pipeline());
					}
					Command::SetMesh(mesh) => {
						render_pass.set_vertex_buffer(0, mesh.get_vertex_buffer(), 0, 0);
						render_pass.set_index_buffer(mesh.get_index_buffer(), 0, 0);
					}
					Command::SetBind(bind, pos) => {
						render_pass.set_bind_group(*pos, bind.get_bind_group(), &[]);
					}
					Command::Draw(range) => {
						render_pass.draw_indexed(range.clone(), 0, 0..1);
					}
					_ => ()
				}
			}
		}
		renderer.get_queue().submit(&[encoder.finish()]);
	}
}