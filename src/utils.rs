use std::time::Duration;

use crate::*;



pub fn place_text_in_box(
	lines: &[(&str, (u8, u8, u8))],
	border_colors: (u8, u8, u8),
	pos: (u16, u16),
	alignment: (f32, f32),
	charmap: &mut charmap_renderer::Charmap
) {
	
	let mut longest_len = 0;
	for (line, _colors) in lines {
		longest_len = longest_len.max(line.len());
	}
	let left_pos = (pos.0 as f32 - (longest_len + 2) as f32 * alignment.0) as u16;
	let top_pos = (pos.1 as f32 - (lines.len() + 2) as f32 * alignment.1) as u16;
	
	for (i, (line, colors)) in lines.iter().enumerate() {
		place_text(line, (left_pos + 1, top_pos + i as u16 + 1), *colors, charmap);
	}
	place_border(left_pos, left_pos + longest_len as u16 + 1, top_pos, top_pos + lines.len() as u16 + 1, border_colors, charmap);
	
}

pub fn place_text(
	text: impl AsRef<str>,
	pos: (u16, u16),
	colors: (u8, u8, u8),
	charmap: &mut charmap_renderer::Charmap,
) -> u16 {
	let mut i = 0;
	for c in text.as_ref().chars() {
		charmap.place(c as u8, (pos.0 + i, pos.1), colors);
		i += 1;
	}
	i
}

pub fn place_border(
	left_edge: u16,
	right_edge: u16,
	top_edge: u16,
	bottom_edge: u16,
	colors: (u8, u8, u8),
	charmap: &mut charmap_renderer::Charmap,
) {
	for x in left_edge + 1..=right_edge - 1 {
		charmap.place(CHAR_EDGE_HORIZONTAL, (x, top_edge), colors);
		charmap.place(CHAR_EDGE_HORIZONTAL, (x, bottom_edge), colors);
	}
	for y in top_edge + 1..=bottom_edge - 1 {
		charmap.place(CHAR_EDGE_VERTICAL, (left_edge, y), colors);
		charmap.place(CHAR_EDGE_VERTICAL, (right_edge, y), colors);
	}
	charmap.place(CHAR_CORNER_TOP, (left_edge, top_edge), colors);
	charmap.place(CHAR_CORNER_TOP, (right_edge, top_edge), colors);
	charmap.place(CHAR_CORNER_BOTTOM, (left_edge, bottom_edge), colors);
	charmap.place(CHAR_CORNER_BOTTOM, (right_edge, bottom_edge), colors);
}



pub fn display_message(message: impl Into<String>, current_message: &mut CurrentMessage) {
	current_message.message = message.into();
	current_message.disappear_time = Instant::now() + Duration::from_secs(3);
	current_message.colors = (COLOR_DARK_GREY, COLOR_WHITE);
}

pub fn display_warning(message: impl Into<String>, current_message: &mut CurrentMessage) {
	current_message.message = message.into();
	current_message.disappear_time = Instant::now() + Duration::from_secs(3);
	current_message.colors = (COLOR_WARNING_YELLOW, COLOR_BLACK);
}

pub fn display_error(message: impl Into<String>, current_message: &mut CurrentMessage) {
	current_message.message = message.into();
	current_message.disappear_time = Instant::now() + Duration::from_secs(3);
	current_message.colors = (COLOR_ERROR_RED, COLOR_WHITE);
}



pub fn get_assets_path() -> Result<PathBuf> {
	let mut output = std::env::current_exe().context("Failed to get the path of the current program binary")?;
	loop {
		output.push("assets");
		if output.exists() {
			return Ok(output);
		}
		output.pop();
		if !output.pop() {
			bail!("Failed to find assets folder near executable or any parent folders.");
		}
	}
}

pub fn last_modified_time(path: &Path) -> Result<SystemTime> {
	path
		.metadata()
		.and_then(|metadata| metadata.modified())
		.with_context(|| format!("Failed to get the metadata for the file {}", path.display()))
}
