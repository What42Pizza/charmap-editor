use crate::*;



const RIGHT_PANEL_SIZE: u16 = 16;
const RIGHT_PANEL_EDGE: u16 = SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 4;
const TOP_PANEL_EDGE: u16 = 4;

const WORK_AREA_LEFT_EDGE: u16 = 2;
const WORK_AREA_RIGHT_EDGE: u16 = RIGHT_PANEL_EDGE - 3;
const WORK_AREA_TOP_EDGE: u16 = TOP_PANEL_EDGE + 2;
const WORK_AREA_BOTTOM_EDGE: u16 = SCREEN_CHAR_HEIGHT - 3;



pub fn update_ui_charmap(data: &mut ProgramData) {
	data.ui_charmap.buffer.fill(1);
	place_border(0, SCREEN_CHAR_WIDTH - 1, 0, SCREEN_CHAR_HEIGHT - 1, (COLOR_BLACK, COLOR_BRIGHT_RED, 0), &mut data.ui_charmap);
	
	place_basic_edges(data);
	place_colors_and_chars(data);
	fill_work_area_with_transparency(data);
	draw_cursor(data);
	
	let prioritized_action = current_prioritized_action(data);
	
	place_button_text("Press H to show controls", (1, 1), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	match prioritized_action {
		CurrentPrioritizedAction::SettingSavePath => {
			show_set_save_path_screen(data);
		}
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
	
	// work area border
	place_border(WORK_AREA_LEFT_EDGE, WORK_AREA_RIGHT_EDGE, WORK_AREA_TOP_EDGE, WORK_AREA_BOTTOM_EDGE, (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	if data.edit_charmap_data.is_none() {
		place_text(" No charmap is loaded ", (TOP_PANEL_EDGE, 6), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	}
	
	// right panel border
	for y in 1 .. SCREEN_CHAR_HEIGHT - 1 {
		data.ui_charmap.place(CHAR_EDGE_VERTICAL, (RIGHT_PANEL_EDGE, y), (COLOR_BLACK, COLOR_WHITE, 0));
	}
	
	// top panel border
	for x in 1 .. RIGHT_PANEL_EDGE {
		data.ui_charmap.place(CHAR_EDGE_HORIZONTAL, (x, TOP_PANEL_EDGE), (COLOR_BLACK, COLOR_WHITE, 0));
	}
	data.ui_charmap.place(CHAR_EDGE_RIGHT_INTERSECT, (RIGHT_PANEL_EDGE, TOP_PANEL_EDGE),(COLOR_BLACK, COLOR_WHITE, 0));
	
}

pub fn place_colors_and_chars(data: &mut ProgramData) {
	
	place_text("Colors:", (RIGHT_PANEL_EDGE + 2, 2), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(RIGHT_PANEL_EDGE + 1, RIGHT_PANEL_EDGE + 18, 3, 20, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut color = 0;
	for y in 4..20 {
		for x in RIGHT_PANEL_EDGE + 2 .. RIGHT_PANEL_EDGE + 18 {
			data.ui_charmap.place(1, (x, y), (color, color, color));
			color = color.wrapping_add(1);
		}
	}
	
	place_text("Background:"  , (RIGHT_PANEL_EDGE + 2, 21), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_text("Foreground 1:", (RIGHT_PANEL_EDGE + 2, 22), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_text("Foreground 2:", (RIGHT_PANEL_EDGE + 2, 23), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	if let Some(edit_charmap_data) = &data.edit_charmap_data {
		let colors = edit_charmap_data.selected_colors;
		place_text("  ", (RIGHT_PANEL_EDGE + 16, 21), (colors.0, colors.0, colors.0), &mut data.ui_charmap);
		place_text("  ", (RIGHT_PANEL_EDGE + 16, 22), (colors.1, colors.1, colors.1), &mut data.ui_charmap);
		place_text("  ", (RIGHT_PANEL_EDGE + 16, 23), (colors.2, colors.2, colors.2), &mut data.ui_charmap);
		data.ui_charmap.place(CHAR_SIDING_LEFT, (RIGHT_PANEL_EDGE + 15, 21), (COLOR_BLACK, COLOR_WHITE, COLOR_BLACK));
		data.ui_charmap.place(CHAR_SIDING_LEFT, (RIGHT_PANEL_EDGE + 15, 22), (COLOR_BLACK, COLOR_WHITE, COLOR_BLACK));
		data.ui_charmap.place(CHAR_SIDING_LEFT, (RIGHT_PANEL_EDGE + 15, 23), (COLOR_BLACK, COLOR_WHITE, COLOR_BLACK));
		data.ui_charmap.place(CHAR_SIDING_RIGHT, (RIGHT_PANEL_EDGE + 18, 21), (COLOR_BLACK, COLOR_WHITE, COLOR_BLACK));
		data.ui_charmap.place(CHAR_SIDING_RIGHT, (RIGHT_PANEL_EDGE + 18, 22), (COLOR_BLACK, COLOR_WHITE, COLOR_BLACK));
		data.ui_charmap.place(CHAR_SIDING_RIGHT, (RIGHT_PANEL_EDGE + 18, 23), (COLOR_BLACK, COLOR_WHITE, COLOR_BLACK));
	}
	
	place_text("Characters:", (RIGHT_PANEL_EDGE + 2, 25), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(RIGHT_PANEL_EDGE + 1, RIGHT_PANEL_EDGE + 18, 26, 43, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut c = 0;
	for y in 27..43 {
		for x in RIGHT_PANEL_EDGE + 2 .. RIGHT_PANEL_EDGE + 18 {
			data.ui_charmap.place(c, (x, y), (COLOR_BLACK, COLOR_WHITE, COLOR_WHITE));
			c = c.wrapping_add(1);
		}
	}
	if let Some(edit_charmap_data) = &data.edit_charmap_data {
		let selected_char = edit_charmap_data.selected_char;
		let x = (selected_char & 0x0F) as u16 + RIGHT_PANEL_EDGE + 2;
		let y = ((selected_char & 0xF0) as u16 >> 4) + 27;
		data.ui_charmap.place(selected_char, (x, y), (COLOR_WHITE, COLOR_ORANGE, COLOR_ORANGE));
	}
	
}

pub fn fill_work_area_with_transparency(data: &mut ProgramData) {
	if data.edit_charmap_data.is_none() { return; }
	let charmap_data = &mut data.ui_charmap.buffer.cpu_copy;
	for y in WORK_AREA_TOP_EDGE + 1 .. WORK_AREA_BOTTOM_EDGE {
		charmap_data[(3 + y * SCREEN_CHAR_WIDTH) as usize * 4 .. (WORK_AREA_RIGHT_EDGE + y * SCREEN_CHAR_WIDTH) as usize * 4].fill(COLOR_TRANSPARENT); // also fills the character id but that's fine
	}
}

pub fn draw_cursor(data: &mut ProgramData) {
	let Some(edit_charmap_data) = &data.edit_charmap_data else { return; };
	let cursor_screen_pos = (
		edit_charmap_data.cursor_pos.0 - edit_charmap_data.view_offset.0 + 3,
		edit_charmap_data.cursor_pos.1 - edit_charmap_data.view_offset.1 + 7,
	);
	data.ui_charmap.place(CHAR_SELECTOR, cursor_screen_pos, (COLOR_TRANSPARENT, COLOR_WHITE, COLOR_WHITE));
	data.ui_charmap.place(CHAR_EDGE_FULL_INTERSECT, (cursor_screen_pos.0, WORK_AREA_TOP_EDGE), (COLOR_BLACK, COLOR_WHITE, 0));
	data.ui_charmap.place(CHAR_EDGE_FULL_INTERSECT, (cursor_screen_pos.0, WORK_AREA_BOTTOM_EDGE), (COLOR_BLACK, COLOR_WHITE, 0));
	data.ui_charmap.place(CHAR_EDGE_FULL_INTERSECT, (WORK_AREA_LEFT_EDGE, cursor_screen_pos.1), (COLOR_BLACK, COLOR_WHITE, 0));
	data.ui_charmap.place(CHAR_EDGE_FULL_INTERSECT, (WORK_AREA_TOP_EDGE, cursor_screen_pos.1), (COLOR_BLACK, COLOR_WHITE, 0));
}



pub fn show_help_screen(data: &mut ProgramData) {
	
	let mut pos = (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 1);
	pos.0 += place_button_text("Open: ctrl+O", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	pos.0 += place_button_text("New: ctrl+N" , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	         place_button_text("Save: ctrl+S", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
	let mut pos = (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 4);
	pos.0 += place_button_text("Close charmap: ctrl+w", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	         place_button_text("Close charmap without saving: shift+ctrl+w", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
	let mut pos = (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 7);
	pos.0 += place_button_text("quit program: ctrl+q"  , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	         place_button_text("toggle fullscreen: f11", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
}

pub fn show_new_charmap_screen(data: &mut ProgramData) {
	let Some(new_charmap_data) = &data.new_charmap_data else { panic!("Cannot call show_new_charmap_screen() when new_charmap_data is empty"); };
	
	let color = if new_charmap_data.selection == NewCharmapSelection::WidthInput { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Width: {}", new_charmap_data.width_input) , (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 1), color, COLOR_BLACK, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::HeightInput { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Height: {}", new_charmap_data.height_input), (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 4), color, COLOR_BLACK, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::CreateButton { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Create Charmap"), (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 7), color, COLOR_BLACK, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::CancelButton { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_button_text(format!("Cancel"), (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 10), color, COLOR_BLACK, &mut data.ui_charmap);
	
}

pub fn show_set_save_path_screen(data: &mut ProgramData) {
	let Some(set_save_path_data) = &data.set_save_path_data else { panic!("Cannot call show_set_save_path_screen() when set_save_path_data is empty"); };
	
	place_button_text("Enter name of file to save as:", (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 1), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	place_button_text(&set_save_path_data.curr_path   , (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 4), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	place_button_text("Press enter or esc when ready" , (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 7), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
	
}
