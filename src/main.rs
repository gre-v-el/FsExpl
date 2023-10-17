mod controls;
mod helper;
mod node;

use std::{env, path::Path};

use macroquad::{prelude::*, ui::{widgets::{Window, Group}, root_ui, hash}};
use node::Node;

#[macroquad::main("fsexpl")]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let mut controls = controls::Controls::new();
	
	let mut denied = Vec::new();

	let mut root = Node::new(Path::new("C:/Users/Public/"), Rect::new(0.0, 0.0, 1.0, 1.0), &mut denied).unwrap();

	let mut last_mouse_move = 0.0;

	loop {
		if mouse_delta_position() != vec2(0.0, 0.0) {
			last_mouse_move = get_time();
		}

		clear_background(BLACK);

		// world space
		set_camera(controls.camera());
		root.draw();

		controls.update();

		let tooltip = root.handle_mouse(
			*controls.mouse_world(), 
			is_mouse_button_pressed(MouseButton::Left), 
			is_mouse_button_pressed(MouseButton::Right)
		).0;


		// UI space
		set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height())));

		// draw tooltip
		if get_time() - last_mouse_move > 2.0 {
			if let Some(tooltip) = tooltip {
				let measurement = measure_text(&tooltip, None, 16, 1.0);
				let w = measurement.width;
				let h = measurement.height + measurement.offset_y;
	
				Window::new(hash!(), Vec2::from(mouse_position()) + vec2(-w*0.5, -h-10.0), vec2(w + 10.0, h + 1.0))
					.movable(false)
					.titlebar(false)
					.ui(&mut *root_ui(), |ui| {
						ui.label(vec2(0.0, 0.0), &tooltip);
					});
			}
		}
		
		// draw sidebar
		draw_rectangle(0.0, 0.0, 150.0, screen_height(), WHITE);
		Group::new(1, vec2(150.0, screen_height()))
			.ui(&mut *root_ui(), |ui| {
				ui.label(vec2(10.0, 10.0), "Denied:");
				for (i, path) in denied.iter().enumerate() {
					ui.label(vec2(20.0, 20.0 + 10.0 * i as f32), &path.to_string_lossy());
				}
			});

		next_frame().await;
	}
}

/*
TODO:
* ux: empty -> choose path / show all
* asynchronous indexing + progress indication
* sort by size/sort alphabetically/shuffle
* area scales - linear/square/logarithmic
* collapse on RMB release iff no drag
* precompute per-frame calculations
*/