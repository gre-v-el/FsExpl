use std::f32::consts::PI;

use egui_macroquad::macroquad::prelude::*;

use crate::{helper::lerp, node::Node};

const ICON: [Vec2; 7] = [
	vec2(0.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 1.0),
	vec2(1.0, 0.2),
	vec2(0.3, 0.2),
	vec2(0.2, 0.0),
	vec2(0.0, 0.0),
];

pub struct Icon {
	rect: Rect,
	start: Option<f32>,
	end: Option<(f32, f32)>, // time, initial angle
}

impl Icon {
	pub fn new(rect: Rect) -> Self {
		Self {
			rect,
			start: None,
			end: None,
		}
	}

	pub fn trigger_start(&mut self) {
		self.start = Some(get_time() as f32);
	}

	pub fn trigger_end(&mut self) {
		self.end = Some((get_time() as f32, (self.t() * 8.0).sin() * 0.3 - PI/2.0));
	}

	pub fn running(&self) -> bool {
		self.start.is_some()
	}

	fn t(&self) -> f32 {
		get_time() as f32 - self.start.unwrap()
	}
	fn t_end(&self) -> f32 {
		(get_time() as f32 - self.end.unwrap().0).powf(0.5)
	}

	pub fn draw(&mut self, camera: &Camera2D, node: Option<&Node>) {
		set_camera(&Camera2D {
			zoom: camera.zoom * self.rect.size(),
			target: (camera.target - self.rect.point()) / (self.rect.size()),
			..Default::default()
		});

		let t = self.t() * 8.0;

		let squish = (t*2.0).cos()*0.08 + 1.0;
		let squash = 1.0 / squish;

		for i in 1..ICON.len() {
			draw_line(
				0.5 + (ICON[i-1].x - 0.5) * squash,
				1.0 - (1.0 - ICON[i-1].y) * squish,
				0.5 + (ICON[i].x - 0.5) * squash,
				1.0 - (1.0 - ICON[i].y) * squish,
			0.01, WHITE);
		}
		
		
		let ang = t.sin() * 0.3 - PI/2.0;
		let mut v1x = ang.cos() * 0.8;
		let mut v1y = ang.sin() * 3.0 + 3.3;

		if let Some((_, ang_init)) = self.end {
			let t = self.t_end();

			let v1x_init = ang_init.cos() * 0.8;
			let v1y_init = ang_init.sin() * 3.0 + 3.3;

			v1x = lerp(v1x_init, -0.3, t);
			v1y = lerp(v1y_init, 1.8, t);
		}



		draw_affine_parallelogram(vec3(0.5 - 0.5 * squash, 1.0, 0.0), vec3(1.0*squash, 0.0, 0.0), vec3(v1x * squash, (-1.0 + v1y) * squish, 0.0), None, BLACK);
		draw_line(
			0.5 - 0.5 * squash,
			1.0, 
			0.5 + (v1x - 0.5) * squash, 
			1.0 - (1.0 - v1y) * squish, 
			0.01, 
			WHITE
		);
		draw_line(
			0.5 + (v1x - 0.5)*squash, 
			1.0 - (1.0 - v1y) * squish, 
			0.5 + (v1x + 0.5)*squash, 
			1.0 - (1.0 - v1y) * squish, 
			0.01, 
			WHITE
		);
		draw_line(
			0.5 + (v1x + 0.5)*squash, 
			1.0 - (1.0 - v1y) * squish, 
			0.5 + 0.5 * squash, 
			1.0, 
			0.01, 
			WHITE
		);
		draw_line(
			0.5 - 0.5 * squash,
			1.0, 
			0.5 + 0.5 * squash, 
			1.0, 
			0.01, 
			WHITE
		);

		set_camera(camera);

		if self.end.is_some() {
			let t = (self.t_end() - 0.5).max(0.0) * 2.0;
			draw_rectangle(0.5 - t*0.5, 0.4 - t*0.4, t, t, node.unwrap().color());

			
			if self.t_end() > 1.0 {
				self.start = None;
				self.end = None;
			}
		}
	}
}