mod controls;
mod helper;
mod node;
mod tree;

use std::{env, path::{Path, PathBuf}};

use egui_macroquad::{macroquad, egui::{Window, Pos2, TextEdit, ScrollArea}};
use macroquad::prelude::*;
use node::Node;

#[macroquad::main("fsexpl")]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let mut controls = controls::Controls::new();
	
	let mut denied: Vec<PathBuf> = Vec::new();

	let mut root: Option<Node> = None;

	let mut last_mouse_move = 0.0; // used for tooltip
	let mut dragged_since_rmb_down = vec2(0.0, 0.0); // used to dermine if a folder should collapse


	let mut path_input_buffer = String::from("D:/pliki/3d/blender 2018+/");

	loop {
		let mouse_delta = mouse_delta_position();

		if mouse_delta != vec2(0.0, 0.0) || 
		   is_mouse_button_pressed(MouseButton::Left) || 
		   is_mouse_button_pressed(MouseButton::Right) 
		{
			last_mouse_move = get_time();
		}

		clear_background(BLACK);

		// world space
		set_camera(controls.camera());

		if let Some(root) = &root {
			root.draw();
		}
		else {
			draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, 0.05, Color::new(0.3, 0.3, 0.3, 1.0));
		}

		if is_mouse_button_pressed(MouseButton::Right) {
			dragged_since_rmb_down = vec2(0.0, 0.0);
		}
		else if is_mouse_button_down(MouseButton::Right) {
			dragged_since_rmb_down += mouse_delta;
		}
		controls.update();

		// UI space
		set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height())));
		
		// get tooltip
		let mut tooltip = None;
		if let Some(root) = &mut root {
			let tooltip_text = root.handle_mouse(
				*controls.mouse_world(), 
				is_mouse_button_pressed(MouseButton::Left), 
				is_mouse_button_released(MouseButton::Right) && dragged_since_rmb_down.length_squared() == 0.0
			).0;
			
			if get_time() - last_mouse_move > 2.0 {
				if let Some(text) = tooltip_text {
					let tooltip_pos = Vec2::from(mouse_position()) + vec2(10.0, 10.0);

					tooltip = Some((text, tooltip_pos));
				}
			}
		}

		egui_macroquad::ui(|ctx| {
			// draw tooltip
			if let Some((text, pos)) = tooltip {
				Window::new("tooltip")
					.collapsible(false)
					.movable(false)
					.title_bar(false)
					.resizable(false)
					.fixed_pos(Pos2::new(pos.x, pos.y))
					.show(ctx, |ui| {
						ui.label(text);
					});
			}

			// draw sidebar
			Window::new("sidebar")
				.fixed_pos(Pos2::new(0.0, 0.0))
				.fixed_size(egui_macroquad::egui::Vec2::new(200.0, screen_height()))
				.movable(false)
				.resizable(false)
				.title_bar(false)
				.collapsible(false)
				.show(ctx, |ui| {
					ui.label("path:");
					ui.add(TextEdit::singleline(&mut path_input_buffer));

					if ui.button("Scan").clicked() {
						root = Some(Node::new(Path::new(&path_input_buffer), Rect::new(0.0, 0.0, 1.0, 1.0), &mut denied).unwrap());
					}

					ui.label("Denied:");
					ScrollArea::vertical()
						.max_height(100.0)
						.auto_shrink([false; 2])
						.min_scrolled_height(100.0)
						.stick_to_bottom(true)
						.show(ui, |ui| {
							for path in denied.iter() {
								ui.label(path.to_string_lossy());
							}
						});

				});
		});

		egui_macroquad::draw();

		next_frame().await;
	}
}

/*
TODO:
* asynchronous indexing + progress indication
* sort by size/sort alphabetically/shuffle
* area scales - linear/square/logarithmic
* precompute per-frame calculations
*/