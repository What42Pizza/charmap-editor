use crate::*;



pub fn update_ui_charmap(data: &mut ProgramData) {
	data.ui_charmap.buffer.fill(0);
	match &data.state {
		ProgramState::FileSelect { current_path } => {
			update_ui_charmap_file_select(&mut data.ui_charmap, &current_path);
		}
		ProgramState::Editing { editing_charmap: _, editing_charmap_path: _ } => {
			update_ui_charmap_editing(&mut data.ui_charmap);
		}
	}
	data.ui_charmap.update_gpu(&data.gpu_instance);
}



pub fn update_ui_charmap_editing(ui_charmap: &mut charmap_renderer::Charmap) {
	
	
	
	place_border(0, SCREEN_CHAR_WIDTH - 1, 0, SCREEN_CHAR_HEIGHT - 1, (0, 2, 0), ui_charmap);
}



pub fn update_ui_charmap_file_select(ui_charmap: &mut charmap_renderer::Charmap, current_path: &str) {
	
	place_text_in_box(&[("Charmap Editor", (0, 1, 0))], (0, 1, 0), (SCREEN_CHAR_WIDTH / 2, SCREEN_CHAR_HEIGHT / 2 - 10), (0.5, 0.5), ui_charmap);
	
	place_text_in_box(&[("File to open:", (0, 1, 0)), (current_path, (0, 1, 0))], (0, 4, 0), (SCREEN_CHAR_WIDTH / 2, SCREEN_CHAR_HEIGHT / 2), (0.5, 0.5), ui_charmap);
	
	place_border(0, SCREEN_CHAR_WIDTH - 1, 0, SCREEN_CHAR_HEIGHT - 1, (0, 2, 0), ui_charmap);
}
