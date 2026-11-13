use crate::*;
use sdl3::{EventPump, keyboard, video::FullscreenType};



pub fn handle_events(event_pump: &mut EventPump, window: &mut Window, window_surface: &mut simple_gpu::WindowSurface, data: &mut ProgramData) {
	
	let keyboard = event_pump.keyboard_state();
	let ctrl_down = keyboard.is_scancode_pressed(keyboard::Scancode::LCtrl) || keyboard.is_scancode_pressed(keyboard::Scancode::RCtrl);
	data.show_help = keyboard.is_scancode_pressed(keyboard::Scancode::H);
	
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
				try_quit(data);
			}
			
			Event::KeyDown { keycode: Some(keycode), .. } => {
				let result = handle_key_down(keycode, ctrl_down, data, window);
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



pub fn handle_key_down(keycode: Keycode, ctrl_down: bool, data: &mut ProgramData, window: &mut Window) -> Result<()>{
	
	if keycode == Keycode::F11 {
		let fullscreen = window.fullscreen_state() == FullscreenType::True;
		window.set_fullscreen(!fullscreen).context("Failed to toggle fullscreen")?;
	}
	
	if ctrl_down && keycode == Keycode::Q {
		try_quit(data);
	}
	
	if ctrl_down && keycode == Keycode::S {
		if let Some(edit_charmap_data) = &mut data.edit_charmap_data {
			
		} else {
			display_error("Cannot save, no charmap is loaded", &mut data.current_message);
		}
	}
	
	Ok(())
}



pub fn try_quit(data: &mut ProgramData) {
	if let Some(edit_charmap_data) = &data.edit_charmap_data && !edit_charmap_data.is_saved {
		display_warning("Cannot quit, charmap is not saved", &mut data.current_message);
		return;
	}
	println!("closing");
	data.should_quit = true;
}
