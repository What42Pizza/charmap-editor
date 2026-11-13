use crate::*;



pub struct MainInitData {
	pub assets_path: PathBuf,
	pub window: sdl3::video::Window,
	pub window_size: (u32, u32),
	pub event_pump: sdl3::EventPump,
	pub gpu_instance: simple_gpu::GpuInstance,
}

pub fn main_init() -> Result<MainInitData> {
	// enable logging
	if std::env::var("RUST_LOG").is_err() {
		unsafe {
			// safety: this seems to only be unsafe if other threads might be reading/writing env vars, and that should not be possible yet since this is the start of the program
			std::env::set_var("RUST_LOG", "warn");
		}
	}
	env_logger::init();
	
	let assets_path = get_assets_path().context("Failed to find path of assets folder")?;
	
	// sdl
	let sdl = sdl3::init().context("Failed to initialize Sdl3")?;
	let video = sdl.video().context("Failed to initialize Sdl3's video subsystem")?;
	let mut window = video
		.window("Character Screen", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
		.position_centered()
		.resizable()
		//.maximized()
		.metal_view()
		.build()
		.context("Failed to create a window with Sdl3")?;
	window.maximize();
	window.subsystem().text_input().start(&window); // allows Event::TextInput to work
	let window_size = window.size();
	let event_pump = sdl.event_pump().context("Failed to create an event pump with Sdl3")?;
	
	let gpu_instance = simple_gpu::init(wgpu::Limits::defaults(), wgpu::MemoryHints::Performance).context("Failed to initialize the graphics stack")?;
	
	Ok(MainInitData {
		assets_path,
		window,
		window_size,
		event_pump,
		gpu_instance,
	})
}



pub fn load_character_atlas(assets_path: &Path, gpu_instance: &simple_gpu::GpuInstance) -> Result<SimpleTex> {
	let path = assets_path.join("char_atlas.png");
	let char_atlas_tex = charmap_renderer::load_character_atlas(&path, gpu_instance)?;
	if char_atlas_tex.settings.size != (160, 224) {
		bail!("Error: the size of the characters atlas (loaded from {}) must be 160x224, but is instead {}x{}", path.display(), char_atlas_tex.settings.size.0, char_atlas_tex.settings.size.1)
	}
	let char_atlas_tex_view = char_atlas_tex.wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
	Ok(SimpleTex {
		tex: char_atlas_tex.wgpu_texture,
		view: char_atlas_tex_view,
		last_modified_time: last_modified_time(&path)?,
		path,
	})
}

pub fn load_colors_lut(assets_path: &Path, gpu_instance: &simple_gpu::GpuInstance) -> Result<SimpleTex> {
	let path = assets_path.join("colors_lut.png");
	let colors_lut_tex = simple_gpu::load_texture_from_path(
		&path,
		wgpu::FilterMode::Nearest,
		1,
		gpu_instance,
	)?;
	if colors_lut_tex.settings.size != (16, 16) {
		bail!("Error: the size of the colors LUT (loaded from {}) must be 16x16, but is instead {}x{}", path.display(), colors_lut_tex.settings.size.0, colors_lut_tex.settings.size.1)
	}
	let colors_lut_tex_view = colors_lut_tex.wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
	Ok(SimpleTex {
		tex: colors_lut_tex.wgpu_texture,
		view: colors_lut_tex_view,
		last_modified_time: last_modified_time(&path)?,
		path,
	})
}
