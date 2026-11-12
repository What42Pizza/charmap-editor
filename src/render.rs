use crate::*;



#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
#[repr(C)]
pub struct RawFinalUniforms {
	
	pub target_size: [u32; 2],
	pub inv_target_size: [f32; 2],
	pub background_color: [u8; 4],
	
	pub fish_eye_amount: f32,
	pub scaling_factor: f32,
	pub scanline_amount: f32,
	pub scanline_scale: f32,
	
	pub padding: [u8; 4],
}

pub fn update_uniforms(data: &mut ProgramData, window_size: (u32, u32)) {
	let mut needs_upload = false;
	let uniforms = &mut data.final_uniforms.cpu_copy;
	
	data.rendering_settings.scaling_factor = SCREEN_HEIGHT as f32 / window_size.1 as f32;
	
	let new_target_size = window_size.into();
	if uniforms.target_size != new_target_size {
		uniforms.target_size = new_target_size;
		uniforms.inv_target_size = [1.0 / new_target_size[0] as f32, 1.0 / new_target_size[1] as f32];
		needs_upload = true;
	}
	
	let new_background_color = [data.rendering_settings.background_color.0, data.rendering_settings.background_color.1, data.rendering_settings.background_color.2, 0];
	if uniforms.background_color != new_background_color {
		uniforms.background_color = new_background_color;
		needs_upload = true;
	}
	
	if uniforms.fish_eye_amount != data.rendering_settings.fish_eye_amount {
		uniforms.fish_eye_amount = data.rendering_settings.fish_eye_amount;
		needs_upload = true;
	}
	
	if uniforms.scaling_factor != data.rendering_settings.scaling_factor {
		uniforms.scaling_factor = data.rendering_settings.scaling_factor;
		needs_upload = true;
	}
	
	if uniforms.scanline_amount != data.rendering_settings.scanline_amount {
		uniforms.scanline_amount = data.rendering_settings.scanline_amount;
		needs_upload = true;
	}
	if uniforms.scanline_scale != data.rendering_settings.scanline_scale {
		uniforms.scanline_scale = data.rendering_settings.scanline_scale;
		needs_upload = true;
	}
	
	if needs_upload {
		simple_gpu::sync_uniforms_buffer(&data.final_uniforms, &data.gpu_instance);
	}
}

pub struct RenderingSettings {
	
	pub background_color: (u8, u8, u8),
	
	pub fish_eye_amount: f32,
	pub scaling_factor: f32,
	pub scanline_amount: f32,
	pub scanline_scale: f32,
	
}



pub fn render(data: &mut ProgramData, window: &Window, window_surface: &mut simple_gpu::WindowSurface) {
	
	update_uniforms(data, window.size());
	
	let (
		surface_tex,
		surface_tex_view,
		mut command_encoder,
	) = match simple_gpu::start_frame("render frame", &window_surface, &data.gpu_instance) {
		simple_gpu::StartFrameResult::Some(tex, view, commands) => (tex, view, commands),
		simple_gpu::StartFrameResult::None => return,
		simple_gpu::StartFrameResult::Error => {
			simple_gpu::reconfigure_window_surface(
				window_surface,
				&data.gpu_instance,
				window.size(),
			);
			return;
		}
	};
	
	// render charmap to intermediate tex
	let mut charmap_render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
		label: Some("Charmap Render Pass"),
		color_attachments: &[Some(wgpu::RenderPassColorAttachment {
			view: &data.intermediate_tex.wgpu_view,
			depth_slice: None,
			resolve_target: None,
			ops: wgpu::Operations {
				load: wgpu::LoadOp::Load,
				store: wgpu::StoreOp::Store,
			},
		})],
		depth_stencil_attachment: None,
		timestamp_writes: None,
		occlusion_query_set: None,
		multiview_mask: None,
	});
	charmap_renderer::render(
		&[&data.ui_charmap],
		&mut charmap_render_pass,
		&mut data.charmap_render_data,
	);
	simple_gpu::finish_render_pass(charmap_render_pass);
	
	// render intermediate tex to screen
	let mut final_render_pass = simple_gpu::start_2d_render_pass(
		"Final Render Pass",
		&data.final_uniforms.wgpu_bind_group,
		&[(&surface_tex_view, None)],
		&mut command_encoder,
	);
	simple_gpu::render(
		&mut final_render_pass,
		&data.final_pipeline,
		&[],
		None,
		&[&data.intermediate_tex.wgpu_bind_group],
		3,
		1,
	);
	simple_gpu::finish_render_pass(final_render_pass);
	
	simple_gpu::finish_frame(command_encoder, surface_tex, &data.gpu_instance);
	
}
