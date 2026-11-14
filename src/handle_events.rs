use crate::*;
use sdl3::{EventPump, keyboard, video::FullscreenType};



pub fn handle_events(event_pump: &mut EventPump, window: &mut Window, window_surface: &mut simple_gpu::WindowSurface, data: &mut ProgramData) {
	
	let keyboard = event_pump.keyboard_state();
	let shift_down = keyboard.is_scancode_pressed(keyboard::Scancode::LShift) || keyboard.is_scancode_pressed(keyboard::Scancode::RShift);
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
			
			Event::Quit { .. } => {
				println!("{event:?}");
				println!("Ctrl+C pressed, forcing close");
				quit(data);
			}
			Event::Window {
				win_event: WindowEvent::CloseRequested,
				..
			} if !data.should_quit => {
				println!("{event:?}");
				try_quit(data);
			}
			
			Event::KeyDown { keycode: Some(keycode), .. } => {
				let result = handle_key_down(keycode, ctrl_down, shift_down, data, window);
				if let Err(err) = result {
					eprintln!("Error while handling event {event:?}: \n{err}");
				}
			}
			
			Event::TextInput { text, .. } => handle_text_input(text, data),
			
			e => {
				info!("Unknown event: {e:?}");
			}
		}
	}
}



pub fn handle_key_down(keycode: Keycode, ctrl_down: bool, shift_down: bool, data: &mut ProgramData, window: &mut Window) -> Result<()>{
	
	if keycode == Keycode::F11 {
		let fullscreen = window.fullscreen_state() == FullscreenType::True;
		window.set_fullscreen(!fullscreen).context("Failed to toggle fullscreen")?;
	}
	if ctrl_down && keycode == Keycode::Q {
		if shift_down {
			quit(data);
		} else {
			try_quit(data);
		}
	}
	if ctrl_down && keycode == Keycode::W {
		if shift_down {
			close_charmap(data);
		} else {
			try_close_charmap(data);
		}
	}
	
	if ctrl_down && keycode == Keycode::S {
		if let Some(edit_charmap_data) = &mut data.edit_charmap_data {
			
		} else {
			display_error("Cannot save, no charmap is loaded", &mut data.current_message);
		}
	}
	if ctrl_down && keycode == Keycode::N {
		if let Some(edit_charmap_data) = &data.edit_charmap_data && !edit_charmap_data.is_saved {
			display_warning("Cannot create a new charmap, the current charmap is not saved", &mut data.current_message);
		} else {
			data.new_charmap_data = Some(NewCharmapData::default());
		}
	}
	
	match current_prioritized_action(data) {
		
		CurrentPrioritizedAction::CreatingNewCharmap => {
			let new_charmap_data = data.new_charmap_data.as_mut().expect("current_prioritized_action() cannot be CreatingNewCharmap when data.new_charmap_data is empty");
			if keycode == Keycode::Escape {
				data.new_charmap_data = None;
				
			} else if keycode == Keycode::Return {
				match new_charmap_data.selection {
					NewCharmapSelection::CreateButton => {
						create_new_editing_charmap(data);
					}
					NewCharmapSelection::CancelButton => {
						data.new_charmap_data = None;
					}
					NewCharmapSelection::WidthInput | NewCharmapSelection::HeightInput => {
						new_charmap_data.selection = new_charmap_data.selection.next_down();
					}
				}
			} else {
				if keycode == Keycode::Up {
					new_charmap_data.selection = new_charmap_data.selection.next_up();
				}
				if keycode == Keycode::Down {
					new_charmap_data.selection = new_charmap_data.selection.next_down();
				}
				if keycode == Keycode::Backspace {
					match new_charmap_data.selection {
						NewCharmapSelection::WidthInput => { new_charmap_data.width_input.pop(); }
						NewCharmapSelection::HeightInput => { new_charmap_data.height_input.pop(); }
						NewCharmapSelection::CreateButton | NewCharmapSelection::CancelButton => {}
					}
				}
			}
		}
		
		CurrentPrioritizedAction::ShowingHelpScreen => {}
		CurrentPrioritizedAction::Nothing => {}
	}
	
	Ok(())
}



pub fn handle_text_input(new_text: String, data: &mut ProgramData) {
	match current_prioritized_action(data) {
		
		CurrentPrioritizedAction::CreatingNewCharmap => {
			let new_charmap_data = data.new_charmap_data.as_mut().expect("current_prioritized_action() cannot be CreatingNewCharmap when data.new_charmap_data is empty");
			match new_charmap_data.selection {
				NewCharmapSelection::WidthInput => new_charmap_data.width_input += &new_text,
				NewCharmapSelection::HeightInput => new_charmap_data.height_input += &new_text,
				NewCharmapSelection::CreateButton | NewCharmapSelection::CancelButton => {}
			}
		}
		
		CurrentPrioritizedAction::ShowingHelpScreen => {}
		CurrentPrioritizedAction::Nothing => {}
	}
}



pub fn create_new_editing_charmap(data: &mut ProgramData) {
	let new_charmap_data = data.new_charmap_data.as_mut().expect("finish_new_charmap() cannot be called when data.new_charmap_data is empty");
	
	let width = match new_charmap_data.width_input.parse::<u16>() {
		StdOk(v) => v,
		StdErr(err) => {
			display_error(format!("Invalid width input: {err}"), &mut data.current_message);
			return;
		}
	};
	let height = match new_charmap_data.height_input.parse::<u16>() {
		StdOk(v) => v,
		StdErr(err) => {
			display_error(format!("Invalid height input: {err}"), &mut data.current_message);
			return;
		}
	};
	
	if width == 0 {
		display_error(format!("Invalid width input: cannot be 0"), &mut data.current_message);
		return;
	}
	if height == 0 {
		display_error(format!("Invalid height input: cannot be 0"), &mut data.current_message);
		return;
	}
	
	let mut editing_charmap = charmap_renderer::Charmap::new("Editing Charmap", (width, height), &data.char_atlas_tex.view, &data.colors_lut_tex.view, &data.charmap_render_data, &data.gpu_instance);
	for (i, v) in editing_charmap.buffer.cpu_copy.iter_mut().enumerate() {
		*v = (i as u8) % 4;
	}
	editing_charmap.update_gpu(&data.gpu_instance);
	
	data.edit_charmap_data = Some(EditCharmapData {
		charmap: editing_charmap,
		save_path: None,
		is_saved: false,
		draw_offset: (0, 0),
		cursor_pos: (0, 0),
		curr_colors: (COLOR_BLACK, COLOR_WHITE, COLOR_WHITE),
	});
	data.new_charmap_data = None;
	
}



pub fn try_quit(data: &mut ProgramData) {
	if let Some(edit_charmap_data) = &data.edit_charmap_data && !edit_charmap_data.is_saved {
		display_warning("Cannot quit, charmap is not saved", &mut data.current_message);
		return;
	}
	quit(data);
}

pub fn quit(data: &mut ProgramData) {
	println!("Closing");
	data.should_quit = true;
}



pub fn try_close_charmap(data: &mut ProgramData) {
	let Some(edit_charmap_data) = &mut data.edit_charmap_data else { return; };
	if !edit_charmap_data.is_saved {
		display_warning("Cannot close charmap, it is not saved", &mut data.current_message);
		return;
	}
	close_charmap(data);
}

pub fn close_charmap(data: &mut ProgramData) {
	data.edit_charmap_data = None;
	display_message("Closed charmap", &mut data.current_message);
}
