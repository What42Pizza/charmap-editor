use crate::*;
use std::fs;
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
			if edit_charmap_data.save_path.is_some() {
				save_charmap(data);
			} else {
				data.set_save_path_data = Some(SetSavePathData { curr_path: String::new() });
			}
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
		
		CurrentPrioritizedAction::SettingSavePath => {
			let set_save_path_data = data.set_save_path_data.as_mut().expect("current_prioritized_action() cannot be SettingSavePath when data.set_save_path_data is empty");
			if keycode == Keycode::Escape {
				data.set_save_path_data = None;
				display_message("Canceled saving charmap", &mut data.current_message);
			} else if keycode == Keycode::Return {
				if let Some(edit_charmap_data) = &mut data.edit_charmap_data {
					if set_save_path_data.curr_path.is_empty() {
						display_error("Cannot save with an empty file name", &mut data.current_message);
					} else {
						edit_charmap_data.save_path = Some(PathBuf::from(set_save_path_data.curr_path.clone()));
						save_charmap(data);
					}
				} else {
					display_error("Cannot save: no charmap is present", &mut data.current_message);
				}
			} else if keycode == Keycode::Backspace {
				set_save_path_data.curr_path.pop();
			}
		}
		
		CurrentPrioritizedAction::CreatingNewCharmap => {
			let new_charmap_data = data.new_charmap_data.as_mut().expect("current_prioritized_action() cannot be CreatingNewCharmap when data.new_charmap_data is empty");
			if keycode == Keycode::Escape {
				data.new_charmap_data = None;
				display_message("Canceled creating new charmap", &mut data.current_message);
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
		
		CurrentPrioritizedAction::SettingSavePath => {
			let set_save_path_data = data.set_save_path_data.as_mut().expect("current_prioritized_action() cannot be SettingSavePath when data.set_save_path_data is empty");
			set_save_path_data.curr_path += &new_text;
		}
		
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
		view_offset: (0, 0),
		cursor_pos: (0, 0),
		selected_char: CHAR_EDGE_HORIZONTAL,
		selected_colors: (COLOR_BLACK, COLOR_WHITE, COLOR_WHITE),
	});
	data.new_charmap_data = None;
	
}



pub fn save_charmap(data: &mut ProgramData) {
	let Some(edit_charmap_data) = &mut data.edit_charmap_data else {
		display_error("Failed to save: no charmap is present", &mut data.current_message);
		return;
	};
	let Some(save_path) = &mut edit_charmap_data.save_path else {
		display_error("Failed to save: charmap does not have any save path", &mut data.current_message);
		return;
	};
	if save_path.is_empty() {
		display_error("Cannot save with an empty file name", &mut data.current_message);
		return;
	}
	save_path.set_extension("dat");
	
	let mut data_to_save = vec![];
	data_to_save.extend_from_slice(&0u16.to_le_bytes()); // first two bytes: format version number (which is 0)
	data_to_save.extend_from_slice(&edit_charmap_data.charmap.size.0.to_le_bytes()); // next two bytes: charmap width
	data_to_save.extend_from_slice(&edit_charmap_data.charmap.size.1.to_le_bytes()); // next two bytes: charmap height
	// next: all charmap data
	data_to_save.extend_from_slice(&edit_charmap_data.charmap);
	
	let result = fs::write(save_path, data_to_save);
	if let StdErr(err) = result {
		display_error(format!("Error saving: {err}"), &mut data.current_message);
	} else {
		edit_charmap_data.is_saved = true;
		data.set_save_path_data = None;
		display_message("Saved successfully", &mut data.current_message);
	}
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
