pub const SCREEN_CHAR_WIDTH: u16 = 128;
pub const SCREEN_CHAR_HEIGHT: u16 = 64;
pub const SCREEN_WIDTH: u16 = SCREEN_CHAR_WIDTH * charmap_renderer::CHAR_WIDTH as u16;
pub const SCREEN_HEIGHT: u16 = SCREEN_CHAR_HEIGHT * charmap_renderer::CHAR_HEIGHT as u16;



pub use anyhow::*;
pub use log::info;
pub use sdl3::{
	event::{Event, WindowEvent},
	keyboard::{KeyboardState, Keycode},
	video::Window,
};
pub use std::{path::{Path, PathBuf}, result::Result::{self as StdResult, Ok as StdOk, Err as StdErr}, time::{Instant, SystemTime}};



pub mod init;
pub use init::*;

pub mod handle_events;
pub use handle_events::*;
pub mod update_ui_charmap;
pub use update_ui_charmap::*;
pub mod render;
pub use render::*;

pub mod indices;
pub use indices::*;
pub mod utils;
pub use utils::*;



pub struct ProgramData {
	
	// program basics
	pub should_quit: bool,
	pub fps_counter: simple_gpu::FpsCounter,
	pub assets_path: PathBuf,
	
	// rendering data
	pub gpu_instance: simple_gpu::GpuInstance,
	pub charmap_render_data: charmap_renderer::CharmapRenderData,
	pub intermediate_tex: simple_gpu::Texture,
	pub final_pipeline: wgpu::RenderPipeline,
	pub final_pipeline_output_format: wgpu::TextureFormat,
	pub final_uniforms: simple_gpu::UniformsBuffer<RawFinalUniforms>,
	pub final_shaders: simple_gpu::Shaders,
	pub rendering_settings: RenderingSettings,
	
	// charmap data
	pub char_atlas_tex: SimpleTex,
	pub colors_lut_tex: SimpleTex,
	pub ui_charmap: charmap_renderer::Charmap,
	pub edit_charmap_data: Option<EditCharmapData>,
	
	// misc
	pub show_help: bool,
	pub current_message: CurrentMessage,
	pub new_charmap_data: Option<NewCharmapData>,
	
}

pub struct SimpleTex {
	pub tex: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub path: PathBuf,
	pub last_modified_time: SystemTime,
}

pub struct EditCharmapData {
	pub data: Vec<u8>,
	pub size: (u16, u16),
	pub save_path: Option<PathBuf>,
	pub is_saved: bool,
	pub cursor_pos: (u16, u16),
}

pub struct CurrentMessage {
	pub message: String,
	pub disappear_time: Instant,
	pub colors: (u8, u8),
}

pub struct NewCharmapData {
	pub width_input: String,
	pub height_input: String,
	pub selection: NewCharmapSelection,
}

#[derive(Copy, Clone, PartialEq)]
pub enum NewCharmapSelection {
	WidthInput,
	HeightInput,
	CreateButton,
}

impl Default for NewCharmapData {
	fn default() -> Self {
		Self {
			width_input: SCREEN_CHAR_WIDTH.to_string(),
			height_input: SCREEN_CHAR_HEIGHT.to_string(),
			selection: NewCharmapSelection::WidthInput,
		}
	}
}

impl NewCharmapSelection {
	pub fn next_down(self) -> Self {
		match self {
			Self::WidthInput => Self::HeightInput,
			Self::HeightInput => Self::CreateButton,
			Self::CreateButton => Self::CreateButton,
		}
	}
	pub fn next_up(self) -> Self {
		match self {
			Self::WidthInput => Self::WidthInput,
			Self::HeightInput => Self::WidthInput,
			Self::CreateButton => Self::HeightInput,
		}
	}
}



#[derive(PartialEq)]
pub enum CurrentPrioritizedAction {
	// ordered from most to least important
	CreatingNewCharmap,
	ShowingHelpScreen,
	Nothing,
}

pub fn current_prioritized_action(data: &ProgramData) -> CurrentPrioritizedAction {
	if data.new_charmap_data.is_some() { return CurrentPrioritizedAction::CreatingNewCharmap; }
	if data.show_help { return CurrentPrioritizedAction::ShowingHelpScreen; }
	CurrentPrioritizedAction::Nothing
}



fn main() {
	let result = main_result();
	if let StdErr(err) = result {
		let mut error = format!("{err}");
		for cause in err.chain().skip(1) {
			error += &format!("\nCaused by: {cause}");
		}
		println!("Fatal error while running program:\n{error}");
		rfd::MessageDialog::new()
			.set_title("Program Crash")
			.set_description(format!("An unrecoverable error has occurred:{error}"))
			.set_level(rfd::MessageLevel::Error)
			.set_buttons(rfd::MessageButtons::Ok)
			.show();
	}
}



fn main_result() -> Result<()> {
	
	// main init
	let MainInitData {
		assets_path,
		mut window,
		window_size,
		mut event_pump,
		mut gpu_instance,
	} = main_init().context("Failed to initialize basic program data")?;
	
	// misc init
	let (window, window_surface) = simple_gpu::get_window_surface_mut(
		&gpu_instance,
		&mut window,
		window_size,
		wgpu::PresentMode::AutoVsync,
	);
	let mut window_surface = window_surface.context("Failed to get the window's drawable surface")?;
	let charmap_render_data = charmap_renderer::make_render_data(
		wgpu::TextureFormat::Rgba8Unorm,
		&gpu_instance,
	);
	
	// textures
	let char_atlas_tex = load_character_atlas(&assets_path, &gpu_instance).context("Failed to load character atlas texture")?;
	let colors_lut_tex = load_colors_lut(&assets_path, &gpu_instance).context("Failed to load colors LUT texture")?;
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
	).context("Failed to load post-processing shader")?;
	let final_pipeline = simple_gpu::create_2d_pipeline("Final Pipeline", &[], &final_shaders, &[window_surface.wgpu_format], &mut gpu_instance);
	let final_uniforms = simple_gpu::create_uniforms_buffer(&gpu_instance);
	
	// make character maps
	let ui_charmap = charmap_renderer::create_charmap("Ui Charmap", (SCREEN_CHAR_WIDTH, SCREEN_CHAR_HEIGHT), &char_atlas_tex.view, &colors_lut_tex.view, &charmap_render_data, &gpu_instance);
	
	// assemble program's data
	let mut data = ProgramData {
		
		should_quit: false,
		fps_counter: simple_gpu::FpsCounter::new(),
		assets_path,
		
		gpu_instance,
		charmap_render_data,
		intermediate_tex,
		final_pipeline,
		final_pipeline_output_format: window_surface.wgpu_format,
		final_uniforms,
		final_shaders,
		rendering_settings: RenderingSettings {
			fish_eye_amount: 0.75,
			scaling_factor: 1.0,
			background_color: (20, 20, 30),
			scanline_amount: 0.5,
			scanline_scale: 4.0,
		},
		
		char_atlas_tex,
		colors_lut_tex,
		ui_charmap,
		edit_charmap_data: None,
		
		show_help: false,
		current_message: CurrentMessage {
			message: String::new(),
			disappear_time: Instant::now(),
			colors: (1, 5),
		},
		new_charmap_data: None,
		
	};
	
	
	
	window.show();
	while !data.should_quit {
		
		handle_events(&mut event_pump, window, &mut window_surface, &mut data);
		update_ui_charmap(&mut data);
		
		render(&mut data, window, &mut window_surface);
		data.fps_counter.tick();
		
		check_and_update_textures(&mut data).context("Failed to check for or load updated input textures")?;
	}
	
	Ok(())
}



pub fn check_and_update_textures(data: &mut ProgramData) -> Result<()> {
	
	let mut needs_rebind = false;
	
	let char_atlas_time = last_modified_time(&data.char_atlas_tex.path).context("Failed to get the last modified time of the character atlas texture")?;
	if char_atlas_time != data.char_atlas_tex.last_modified_time {
		data.char_atlas_tex = load_character_atlas(&data.assets_path, &data.gpu_instance).context("Failed to load character atlas texture")?;
		needs_rebind = true;
		println!("Reloaded the character atlas texture");
	}
	
	let colors_lut_time = last_modified_time(&data.colors_lut_tex.path).context("Failed to get the last modified time of the colors LUT texture")?;
	if colors_lut_time != data.colors_lut_tex.last_modified_time {
		data.colors_lut_tex = load_colors_lut(&data.assets_path, &data.gpu_instance).context("Failed to load colors LUT texture")?;
		needs_rebind = true;
		println!("Reloaded the colors LUT texture");
	}
	
	if needs_rebind {
		data.ui_charmap.update_textures(&data.char_atlas_tex.view, &data.colors_lut_tex.view, &data.charmap_render_data, &data.gpu_instance);
	}
	
	Ok(())
}
