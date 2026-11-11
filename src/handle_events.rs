use crate::*;
use sdl3::EventPump;



pub fn handle_events(event_pump: &mut EventPump, window: &Window, window_surface: &mut simple_gpu::WindowSurface, data: &mut ProgramData) {
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
			}
			| Event::KeyDown {
				keycode: Some(Keycode::Escape),
				..
			} => {
				println!("closing"); // note: this can get both a quit event and a close requested event in the same frame
				data.should_quit = true;
			}
			
			e => {
				info!("Unknown event: {e:?}");
			}
		}
	}
}
