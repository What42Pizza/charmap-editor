use crate::*;
use sdl3::{EventPump, video::FullscreenType};



pub fn handle_events(event_pump: &mut EventPump, window: &mut Window, window_surface: &mut simple_gpu::WindowSurface, data: &mut ProgramData) {
	for event in event_pump.poll_iter() {
		match event {
			
			Event::Window {
				window_id,
				win_event:
					WindowEvent::PixelSizeChanged(new_width, new_height)
					| WindowEvent::Resized(new_width, new_height),
				..
			} if window_id == window.id() => {
				simple_gpu::reconfigure_window_surface(
					window_surface,
					&data.gpu_instance,
					(new_width as u32, new_height as u32),
				);
			}
			
			Event::Quit { .. }
			| Event::Window {
				win_event: WindowEvent::CloseRequested,
				..
			} if !data.should_quit => {
				println!("closing");
				data.should_quit = true;
			}
			
			Event::KeyDown { keycode: Some(keycode), .. } => {
				let result = handle_key_down(keycode, data, window);
				if let Err(err) = result {
					eprintln!("Error while handling event {event:?}: \n{err}");
				}
			}
			
			Event::TextInput { text: _, .. } => {
				
			}
			
			e => {
				info!("Unknown event: {e:?}");
			}
		}
	}
}



pub fn handle_key_down(keycode: Keycode, data: &mut ProgramData, window: &mut Window) -> Result<()>{
	
	if keycode == Keycode::F11 {
		let fullscreen = window.fullscreen_state() == FullscreenType::True;
		window.set_fullscreen(!fullscreen).context("Failed to toggle fullscreen")?;
	}
	
	if keycode == Keycode::Escape {
		println!("closing");
		data.should_quit = true;
	}
	
	Ok(())
}
