use crate::*;



const RIGHT_PANEL_SIZE: u16 = 16;



pub fn update_ui_charmap(data: &mut ProgramData) {
	data.ui_charmap.buffer.fill(0);
	place_border(0, SCREEN_CHAR_WIDTH - 1, 0, SCREEN_CHAR_HEIGHT - 1, (COLOR_BLACK, COLOR_BRIGHT_RED, 0), &mut data.ui_charmap);
	
	place_basic_edges(data);
	place_colors_and_chars(data);
	
	let prioritized_action = current_prioritized_action(data);
	
	place_button_text("Press H to show controls", (1, 1), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	match prioritized_action {
		CurrentPrioritizedAction::CreatingNewCharmap => {
			show_new_charmap_screen(data);
		}
		CurrentPrioritizedAction::ShowingHelpScreen => {
			show_help_screen(data);
		}
		CurrentPrioritizedAction::Nothing => {}
	}
	
	if Instant::now() < data.current_message.disappear_time {
		place_button_text(&data.current_message.message, (27, 1), (data.current_message.colors.0, data.current_message.colors.1, 0), COLOR_BLACK, &mut data.ui_charmap);
	}
	
	data.ui_charmap.update_gpu(&data.gpu_instance);
}



pub fn place_basic_edges(data: &mut ProgramData) {
	
	// editing charmap border
	place_border(2, SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 7, 6, SCREEN_CHAR_HEIGHT - 3, (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	if data.edit_charmap_data.is_none() {
		place_text(" No charmap is loaded ", (4, 6), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	}
	
	// right panel border
	for y in 1 .. SCREEN_CHAR_HEIGHT - 1 {
		data.ui_charmap.place(CHAR_EDGE_VERTICAL, (SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 4, y), (COLOR_BLACK, COLOR_WHITE, 0));
	}
	
	// top panel border
	for x in 1 .. SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 4 {
		data.ui_charmap.place(CHAR_EDGE_HORIZONTAL, (x, 4), (COLOR_BLACK, COLOR_WHITE, 0));
	}
	data.ui_charmap.place(CHAR_EDGE_RIGHT_INTERSECT, (SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 4, 4),(COLOR_BLACK, COLOR_WHITE, 0));
	
}

pub fn place_colors_and_chars(data: &mut ProgramData) {
	
	place_text("Colors:", (SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 2, 2), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 3, SCREEN_CHAR_WIDTH - 2, 3, 20, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut color = 0;
	for y in 4..20 {
		for x in SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 2 .. SCREEN_CHAR_WIDTH - 2 {
			data.ui_charmap.place(1, (x, y), (color, color, 0));
			color = color.wrapping_add(1);
		}
	}
	
	place_text("Characters:", (SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 2, 21), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 3, SCREEN_CHAR_WIDTH - 2, 22, 39, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut c = 0;
	for y in 23..39 {
		for x in SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 2 .. SCREEN_CHAR_WIDTH - 2 {
			data.ui_charmap.place(c, (x, y), (COLOR_BLACK, COLOR_WHITE, COLOR_BRIGHT_YELLOW));
			c = c.wrapping_add(1);
		}
	}
	
}



pub fn show_help_screen(data: &mut ProgramData) {
	
	let mut pos = (3, 7);
	pos.0 += place_button_text("Open: ctrl+O", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	pos.0 += place_button_text("New: ctrl+N" , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	         place_button_text("Save: ctrl+S", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
	let mut pos = (3, 10);
	pos.0 += place_button_text("Close charmap: ctrl+w", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	         place_button_text("Close charmap without saving: shift+ctrl+w", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
	let mut pos = (3, 13);
	pos.0 += place_button_text("quit program: ctrl+q"  , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	         place_button_text("toggle fullscreen: f11", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
}

pub fn show_new_charmap_screen(data: &mut ProgramData) {
	let Some(new_charmap_data) = &data.new_charmap_data else { panic!("Cannot call show_new_charmap_screen() when new_charmap_data is empty"); };
	
	let color = if new_charmap_data.selection == NewCharmapSelection::WidthInput { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Width: {}", new_charmap_data.width_input) , (3,  7), color, COLOR_BLACK, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::HeightInput { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Height: {}", new_charmap_data.height_input), (3, 10), color, COLOR_BLACK, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::CreateButton { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Create Charmap"), (3, 13), color, COLOR_BLACK, &mut data.ui_charmap);
	
}



pub fn place_button_text(text: impl AsRef<str>, pos: (u16, u16), colors: (u8, u8, u8), background_color: u8, charmap: &mut charmap_renderer::Charmap) -> u16 {
	let text_len = place_text(text, (pos.0 + 1, pos.1 + 1), colors, charmap);
	let colors = (background_color, colors.0, colors.0);
	place_siding(pos.0, pos.0 + text_len + 1, pos.1, pos.1 + 2, colors, charmap);
	text_len + 2
}
