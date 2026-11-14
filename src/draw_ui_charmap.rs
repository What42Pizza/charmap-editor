use crate::*;



const RIGHT_PANEL_SIZE: u16 = 34;
const RIGHT_PANEL_EDGE: u16 = SCREEN_CHAR_WIDTH - RIGHT_PANEL_SIZE - 4;
const TOP_PANEL_EDGE: u16 = 4;

const RIGHT_TILE_0_POS: (u16, u16) = (RIGHT_PANEL_EDGE + 1, 2);
const RIGHT_TILE_1_POS: (u16, u16) = (RIGHT_TILE_0_POS.0 + 18, RIGHT_TILE_0_POS.1);
const RIGHT_EXAMPLE_POS: (u16, u16) = (RIGHT_TILE_0_POS.0 + 2, RIGHT_TILE_0_POS.1 + 19);
const RIGHT_TILE_2_POS: (u16, u16) = (RIGHT_TILE_0_POS.0, RIGHT_EXAMPLE_POS.1 + 5);
const RIGHT_TILE_3_POS: (u16, u16) = (RIGHT_TILE_1_POS.0, RIGHT_EXAMPLE_POS.1 + 5);

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
	
	place_bordered_text("Press H to show controls", (1, 1), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_BLACK, &mut data.ui_charmap);
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
		place_bordered_text(&data.current_message.message, (27, 1), (data.current_message.colors.0, data.current_message.colors.1, 0), COLOR_BLACK, &mut data.ui_charmap);
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
	
	place_text("Background Color", (RIGHT_TILE_0_POS.0 + 1, RIGHT_TILE_0_POS.1), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(RIGHT_TILE_0_POS.0, RIGHT_TILE_0_POS.0 + 17, RIGHT_TILE_0_POS.1 + 1, RIGHT_TILE_0_POS.1 + 18, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut i = 0;
	for y in RIGHT_TILE_0_POS.1 + 2 .. RIGHT_TILE_0_POS.1 + 18 {
		for x in RIGHT_TILE_0_POS.0 + 1 .. RIGHT_TILE_0_POS.0 + 17 {
			data.ui_charmap.place(1, (x, y), (i, i, i));
			i = i.wrapping_add(1);
		}
	}
	
	place_text("Foreground Color", (RIGHT_TILE_1_POS.0 + 1, RIGHT_TILE_1_POS.1), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(RIGHT_TILE_1_POS.0, RIGHT_TILE_1_POS.0 + 17, RIGHT_TILE_1_POS.1 + 1, RIGHT_TILE_0_POS.1 + 18, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut i = 0;
	for y in RIGHT_TILE_1_POS.1 + 2 .. RIGHT_TILE_1_POS.1 + 18 {
		for x in RIGHT_TILE_1_POS.0 + 1 .. RIGHT_TILE_1_POS.0 + 17 {
			data.ui_charmap.place(1, (x, y), (i, i, i));
			i = i.wrapping_add(1);
		}
	}
	
	place_text("Example: ", (RIGHT_EXAMPLE_POS.0 + 3, RIGHT_EXAMPLE_POS.1 + 2), (COLOR_BLACK, COLOR_WHITE, COLOR_WHITE), &mut data.ui_charmap);
	place_siding(RIGHT_EXAMPLE_POS.0 + 11, RIGHT_EXAMPLE_POS.0 + 15, RIGHT_EXAMPLE_POS.1, RIGHT_EXAMPLE_POS.1 + 4, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	if let Some(edit_charmap_data) = &data.edit_charmap_data {
		for y in RIGHT_EXAMPLE_POS.1 + 1 .. RIGHT_EXAMPLE_POS.1 + 4 {
			for x in RIGHT_EXAMPLE_POS.0 + 12 .. RIGHT_EXAMPLE_POS.0 + 15 {
				data.ui_charmap.place(edit_charmap_data.selected_char, (x, y), edit_charmap_data.selected_colors);
			}
		}
	}
	
	place_text("Character", (RIGHT_TILE_2_POS.0 + 1, RIGHT_TILE_2_POS.1), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(RIGHT_TILE_2_POS.0, RIGHT_TILE_2_POS.0 + 17, RIGHT_TILE_2_POS.1 + 1, RIGHT_TILE_2_POS.1 + 18, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut i = 0;
	for y in RIGHT_TILE_2_POS.1 + 2 .. RIGHT_TILE_2_POS.1 + 18 {
		for x in RIGHT_TILE_2_POS.0 + 1 .. RIGHT_TILE_2_POS.0 + 17 {
			data.ui_charmap.place(i, (x, y), (COLOR_BLACK, COLOR_WHITE, COLOR_BRIGHT_ORANGE));
			i = i.wrapping_add(1);
		}
	}
	
	place_text("Detail Color", (RIGHT_TILE_3_POS.0 + 1, RIGHT_TILE_3_POS.1), (COLOR_BLACK, COLOR_WHITE, 0), &mut data.ui_charmap);
	place_siding(RIGHT_TILE_3_POS.0, RIGHT_TILE_3_POS.0 + 17, RIGHT_TILE_3_POS.1 + 1, RIGHT_TILE_3_POS.1 + 18, (COLOR_BLACK, COLOR_DARK_GREY, COLOR_LIGHT_GREY), &mut data.ui_charmap);
	let mut i = 0;
	for y in RIGHT_TILE_3_POS.1 + 2 .. RIGHT_TILE_3_POS.1 + 18 {
		for x in RIGHT_TILE_3_POS.0 + 1 .. RIGHT_TILE_3_POS.0 + 17 {
			data.ui_charmap.place(1, (x, y), (i, i, i));
			i = i.wrapping_add(1);
		}
	}
	
	//if let Some(edit_charmap_data) = &data.edit_charmap_data {
	//	let selected_char = edit_charmap_data.selected_char;
	//	let x = (selected_char & 0x0F) as u16 + RIGHT_PANEL_EDGE + 2;
	//	let y = ((selected_char & 0xF0) as u16 >> 4) + 27;
	//	data.ui_charmap.place(selected_char, (x, y), (COLOR_WHITE, COLOR_BRIGHT_ORANGE, COLOR_BRIGHT_MAGENTA));
	//}
	
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
	data.ui_charmap.place(CHAR_EDGE_FULL_INTERSECT, (WORK_AREA_RIGHT_EDGE, cursor_screen_pos.1), (COLOR_BLACK, COLOR_WHITE, 0));
}



pub fn show_help_screen(data: &mut ProgramData) {
	
	let mut pos = (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 1);
	pos.0 += place_bordered_text("Open: ctrl+O", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	pos.0 += place_bordered_text("New: ctrl+N" , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	         place_bordered_text("Save: ctrl+S", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	
	let mut pos = (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 4);
	pos.0 += place_bordered_text("Close charmap: ctrl+w", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	         place_bordered_text("Close charmap without saving: shift+ctrl+w", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	
	let mut pos = (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 7);
	pos.0 += place_bordered_text("quit program: ctrl+q"  , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	pos.0 += place_bordered_text("quit program without saving: shift+ctrl+q"  , pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	         place_bordered_text("toggle fullscreen: f11", pos, (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	
}

pub fn show_new_charmap_screen(data: &mut ProgramData) {
	let Some(new_charmap_data) = &data.new_charmap_data else { panic!("Cannot call show_new_charmap_screen() when new_charmap_data is empty"); };
	
	place_bordered_text("Input size of new charmap:", (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 1), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::WidthInput { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_bordered_text(format!("Width: {}", new_charmap_data.width_input) , (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 4), color, COLOR_TRANSPARENT, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::HeightInput { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_bordered_text(format!("Height: {}", new_charmap_data.height_input), (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 7), color, COLOR_TRANSPARENT, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::CreateButton { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_bordered_text(format!("Create Charmap"), (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 10), color, COLOR_TRANSPARENT, &mut data.ui_charmap);
	
	let color = if new_charmap_data.selection == NewCharmapSelection::CancelButton { (COLOR_WHITE, COLOR_BLACK, 0) } else { (COLOR_DARK_GREY, COLOR_WHITE, 0) };
	place_bordered_text(format!("Cancel"), (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 13), color, COLOR_TRANSPARENT, &mut data.ui_charmap);
	
}

pub fn show_set_save_path_screen(data: &mut ProgramData) {
	let Some(set_save_path_data) = &data.set_save_path_data else { panic!("Cannot call show_set_save_path_screen() when set_save_path_data is empty"); };
	
	place_bordered_text("Enter name of file to save as:", (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 1), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	place_bordered_text(&set_save_path_data.curr_path   , (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 4), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	place_bordered_text("Press enter or esc when ready" , (WORK_AREA_LEFT_EDGE + 1, WORK_AREA_TOP_EDGE + 7), (COLOR_DARK_GREY, COLOR_WHITE, 0), COLOR_TRANSPARENT, &mut data.ui_charmap);
	
}
