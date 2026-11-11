//#![allow(unused)]
//#![warn(unused_must_use)]



pub const SCREEN_CHAR_WIDTH: u16 = 150;
pub const SCREEN_CHAR_HEIGHT: u16 = 70;
pub const SCREEN_WIDTH: u16 = SCREEN_CHAR_WIDTH * charmap_renderer::CHAR_WIDTH as u16;
pub const SCREEN_HEIGHT: u16 = SCREEN_CHAR_HEIGHT * charmap_renderer::CHAR_HEIGHT as u16;



pub use anyhow::*;
pub use log::info;
pub use sdl3::{
	event::{Event, WindowEvent},
	keyboard::{KeyboardState, Keycode},
	video::Window,
};
pub use std::{path::{Path, PathBuf}, time::{Instant, SystemTime}};



pub mod init;
pub use init::*;

pub mod handle_events;
pub use handle_events::*;
pub mod render;
pub use render::*;

pub mod utils;
pub use utils::*;



pub struct ProgramData {
	
	// program basics
	pub should_quit: bool,
	pub fps_counter: simple_gpu::FpsCounter,
	pub assets_path: PathBuf,
	
	// rendering data
	pub gpu_instance: simple_gpu::GpuInstance,
	pub char_atlas_tex: SimpleTex,
	pub colors_lut_tex: SimpleTex,
	pub intermediate_tex: simple_gpu::Texture,
	pub final_pipeline: wgpu::RenderPipeline,
	pub final_pipeline_output_format: wgpu::TextureFormat,
	pub final_uniforms: simple_gpu::UniformsBuffer<RawFinalUniforms>,
	pub final_shaders: simple_gpu::Shaders,
	pub rendering_settings: RenderingSettings,
	
	// charmap data
	pub main_charmap: charmap_renderer::Charmap,
	pub charmap_render_data: charmap_renderer::CharmapRenderData,
	
	
}

pub struct SimpleTex {
	pub tex: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub path: PathBuf,
	pub last_modified_time: SystemTime,
}



fn main() -> Result<()> {
	
	// main init
	let MainInitData {
		assets_path,
		mut window,
		window_size,
		mut event_pump,
		mut gpu_instance,
	} = main_init()?;
	
	// misc init
	let (window, window_surface) = simple_gpu::get_window_surface_mut(
		&gpu_instance,
		&mut window,
		window_size,
		wgpu::PresentMode::AutoVsync,
	);
	let mut window_surface = window_surface?;
	let charmap_render_data = charmap_renderer::make_render_data(
		wgpu::TextureFormat::Rgba8Unorm,
		&gpu_instance,
	);
	
	// textures
	let char_atlas_tex = load_character_atlas(&assets_path, &gpu_instance)?;
	let colors_lut_tex = load_colors_lut(&assets_path, &gpu_instance)?;
	let intermediate_tex = simple_gpu::create_texture("Intermediate Render Texture", (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32), wgpu::TextureFormat::Rgba8Unorm, wgpu::FilterMode::Linear, 1, &gpu_instance);
	
	// final render pass
	let final_shaders = simple_gpu::load_glsl_shaders(
		assets_path.join("shaders/final.vsh"),
		assets_path.join("shaders/final.fsh"),
		&gpu_instance,
		&[
			("CHAR_WIDTH", &charmap_renderer::CHAR_WIDTH.to_string()),
			("CHAR_HEIGHT", &charmap_renderer::CHAR_HEIGHT.to_string()),
			("SCREEN_CHAR_WIDTH", &SCREEN_CHAR_WIDTH.to_string()),
			("SCREEN_CHAR_HEIGHT", &SCREEN_CHAR_HEIGHT.to_string()),
		]
	)?;
	let final_pipeline = simple_gpu::create_2d_pipeline("Final Pipeline", &[], &final_shaders, &[window_surface.wgpu_format], &mut gpu_instance);
	let final_uniforms = simple_gpu::create_uniforms_buffer(&gpu_instance);
	
	// make character map
	let mut main_charmap = charmap_renderer::create_charmap("Main Charmap", (SCREEN_CHAR_WIDTH, SCREEN_CHAR_HEIGHT), &char_atlas_tex.view, &colors_lut_tex.view, &charmap_render_data, &gpu_instance);
	let all_chars_1 = (0u8..=63).map(|i| i as char).collect::<String>();
	let all_chars_2 = (64u8..=127).map(|i| i as char).collect::<String>();
	let all_chars_3 = (128u8..=191).map(|i| i as char).collect::<String>();
	let all_chars_4 = (192u8..=255).map(|i| i as char).collect::<String>();
	place_text_in_box(
		&[
			("HELLO, WORLD!", (0, 1, 0)),
			("The quick brown fox jumped over the lazy dog.", (0, 1, 0)),
			("", (0, 1, 0)),
			("this is a test of how much text can actually be put on the screen before it completely fills up", (0, 1, 0)),
			("1+2-3%4+5+6+7+8+9+0=1234567890", (0, 1, 0)),
			("this is \"quoted text\" and 'single quotes'", (0, 1, 0)),
			("100%&25%^30%$43%*78%", (0, 1, 0)),
			("", (0, 1, 0)),
			("all chars:", (0, 1, 0)),
			(&all_chars_1, (0, 1, 0)),
			(&all_chars_2, (0, 1, 0)),
			(&all_chars_3, (0, 1, 0)),
			(&all_chars_4, (0, 1, 0)),
			("", (0, 1, 0)),
			("Even more text!", (0, 1, 0)),
		],
		(0, 4, 0),
		(main_charmap.size.0 / 2, main_charmap.size.1 / 2),
		(0.5, 0.5),
		&mut main_charmap,
	);
	place_border(0, SCREEN_CHAR_WIDTH-1, 0, SCREEN_CHAR_HEIGHT-1, (0, 2, 0), &mut main_charmap);
	main_charmap.update_gpu(&gpu_instance);
	
	// assemble program's data
	let mut data = ProgramData {
		
		should_quit: false,
		fps_counter: simple_gpu::FpsCounter::new(),
		assets_path,
		
		gpu_instance,
		char_atlas_tex,
		colors_lut_tex,
		intermediate_tex,
		final_pipeline,
		final_pipeline_output_format: window_surface.wgpu_format,
		final_uniforms,
		final_shaders,
		rendering_settings: RenderingSettings {
			fish_eye_amount: 0.85,
			scaling_factor: 1.0,
			background_color: (20, 20, 30),
			scanline_amount: 0.5,
			scanline_scale: 4.0,
		},
		
		charmap_render_data,
		main_charmap,
		
	};
	
	
	
	window.show();
	while !data.should_quit {
		
		handle_events(&mut event_pump, window, &mut window_surface, &mut data);
		
		render(&mut data, window, &mut window_surface);
		data.fps_counter.tick();
		
		check_and_update_textures(&mut data)?;
	}
	
	Ok(())
}



pub fn check_and_update_textures(data: &mut ProgramData) -> Result<()> {
	
	let mut needs_rebind = false;
	
	if last_modified_time(&data.char_atlas_tex.path)? != data.char_atlas_tex.last_modified_time {
		data.char_atlas_tex = load_character_atlas(&data.assets_path, &data.gpu_instance)?;
		needs_rebind = true;
		println!("Reloaded the character atlas texture");
	}
	if last_modified_time(&data.colors_lut_tex.path)? != data.colors_lut_tex.last_modified_time {
		data.colors_lut_tex = load_colors_lut(&data.assets_path, &data.gpu_instance)?;
		needs_rebind = true;
		println!("Reloaded the colors LUT texture");
	}
	
	if needs_rebind {
		data.main_charmap.update_textures(&data.char_atlas_tex.view, &data.colors_lut_tex.view, &mut data.charmap_render_data, &data.gpu_instance);
	}
	
	Ok(())
}
