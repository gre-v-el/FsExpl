use std::path::{PathBuf, Path};

use egui_macroquad::{macroquad::prelude::*, egui::{Pos2, TextEdit, Window, ScrollArea}};
use crate::{controls::Controls, tree::Tree, helper::{draw_centered_text, bytes_to_text}, icon::Icon};


pub struct State {
	controls: Controls,
	denied: Vec<PathBuf>,
	tree: Tree,
	last_mouse_move: f64, // determine if should show tooltip
	dragged_since_rmb_down: Vec2, // determine if hovered folder should collapse
	path_input_buffer: String,
	icon: Icon,
}

impl State {
	pub fn new() -> Self {
		Self {
			icon: Icon::new(Rect::new(0.25, 0.2, 0.5, 0.3)),
			controls: Controls::new(),
			denied: Vec::new(),
			tree: Tree::Empty,
			last_mouse_move: 0.0,
			dragged_since_rmb_down: vec2(0.0, 0.0),
			path_input_buffer: "C:".into(),
		}
	}

	pub fn frame(&mut self) {
		if self.tree.process(&mut self.denied) {
			self.icon.trigger_end();
		}

		self.update_interactions();		

		clear_background(BLACK);

		self.draw_tree();
				
		let tooltip = self.tree_handle_mouse();
		self.draw_ui(tooltip);
	}

	fn update_interactions(&mut self) {
		let mouse_delta = mouse_delta_position();

		if mouse_delta != vec2(0.0, 0.0) || 
		   is_mouse_button_pressed(MouseButton::Left) || 
		   is_mouse_button_pressed(MouseButton::Right) 
		{
			self.last_mouse_move = get_time();
		}

		if is_mouse_button_pressed(MouseButton::Right) {
			self.dragged_since_rmb_down = vec2(0.0, 0.0);
		}
		else if is_mouse_button_down(MouseButton::Right) {
			self.dragged_since_rmb_down += mouse_delta;
		}

		self.controls.update();
	}

	fn tree_handle_mouse(&mut self) -> Option<(String, Vec2)> {
		let mut tooltip = None;
		if let Tree::Ready(root, _, _) = &mut self.tree {
			let tooltip_text = root.handle_mouse(
				*self.controls.mouse_world(), 
				is_mouse_button_pressed(MouseButton::Left), 
				is_mouse_button_released(MouseButton::Right) && self.dragged_since_rmb_down.length_squared() == 0.0
			).0;
			
			if get_time() - self.last_mouse_move > 2.0 {
				if let Some(text) = tooltip_text {
					let tooltip_pos = Vec2::from(mouse_position()) + vec2(10.0, 10.0);

					tooltip = Some((text, tooltip_pos));
				}
			}
		}

		tooltip
	} 

	fn draw_tree(&mut self) {
		set_camera(self.controls.camera());

		if let Tree::Empty = &self.tree {
			draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, 0.05, Color::new(0.3, 0.3, 0.3, 1.0));
		}
		else if self.icon.running() {
			if let Tree::Processing(bytes, files, _) = &self.tree {
				draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, 0.05, Color::new(0.5, 0.5, 0.5, 1.0));
				draw_centered_text(&bytes_to_text(*bytes), 0.15, vec2(0.5, 0.7));
				draw_centered_text(&format!("{files} files"), 0.15, vec2(0.5, 0.85));

				self.icon.draw(&self.controls.camera(), None);
			}
			else if let Tree::Ready(node, bytes, files) = &self.tree {
				draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, 0.05, Color::new(0.5, 0.5, 0.5, 1.0));
				draw_centered_text(&bytes_to_text(*bytes), 0.15, vec2(0.5, 0.7));
				draw_centered_text(&format!("{files} files"), 0.15, vec2(0.5, 0.85));

				self.icon.draw(&self.controls.camera(), Some(node));
			}
		}
		else if let Tree::Ready(root, _, _) = &self.tree {
			root.draw();
		}
	}

	fn draw_ui(&mut self, tooltip: Option<(String, Vec2)>) {
		set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height())));

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
				.default_size(egui_macroquad::egui::Vec2::new(200.0, screen_height()))
				.movable(false)
				// .resizable(false)
				.title_bar(false)
				.collapsible(false)
				.show(ctx, |ui| {
					ui.label("path:");
					ui.add(TextEdit::singleline(&mut self.path_input_buffer));

					if ui.button("Scan").clicked() {
						self.tree.scan(Path::new(&self.path_input_buffer));
						self.icon.trigger_start();
					}

					ui.label("Denied:");
					ScrollArea::vertical()
						.max_height(200.0)
						.auto_shrink([false; 2])
						.min_scrolled_height(200.0)
						.stick_to_bottom(true)
						.show(ui, |ui| {
							for path in self.denied.iter() {
								ui.label(path.to_string_lossy());
							}
						});

				});
		});

		egui_macroquad::draw();
	}
}