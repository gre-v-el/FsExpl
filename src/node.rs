use std::{fs, path::{PathBuf, Path}};

use egui_macroquad::macroquad;
use macroquad::prelude::*;

use crate::helper::{random_col, bytes_to_text, shrink_rect_margin};

#[derive(Debug)]
pub struct Node {
	path_prefix: String,
	name: String,
	bytes: u64,
	children: Vec<Node>,
	big_rect: Rect,
	small_rect: Rect,
	color: Color,
	hovered: bool,
	is_leaf: bool,
}

impl Node {
	pub fn new(path: &Path, rect: Rect, denied: &mut Vec<PathBuf>) -> Option<Self> {

		let mut small_rect = rect.clone();
		shrink_rect_margin(&mut small_rect, 0.05);

		let mut pre_path = path.to_string_lossy().to_string();
		let mut name = None;

		if pre_path.ends_with('/') {
			pre_path.pop();
		}
		
		let mut last_slash = None;
		for (i, ch) in pre_path.chars().enumerate() {
			if ch == '/' || ch == '\\' {
				last_slash = Some(i);
			}
		}
		if let Some(last) = last_slash {
			name = Some(pre_path.split_off(last + 1));
		}
		
		let mut children = Vec::new();
		let mut bytes = 0;

		if path.is_file() {
			let metadata = match path.metadata() {
				Ok(m) => m,
				Err(_) => {
					denied.push(path.to_owned());
					return None;
				}
			};
			bytes = metadata.len();
		}
		else {
			let iterator = match fs::read_dir(path) {
				Ok(i) => i,
				Err(_) => {
					denied.push(path.to_path_buf());
					return None;
				},
			};
	
			for entry in iterator {
				let entry = match entry {
					Ok(e) => e,
					Err(_) => {
						denied.push(path.to_path_buf());
						continue;
					}
				};

				let child = Node::new(&entry.path(), Rect::new(1.0, 1.0, 1.0, 1.0), denied);
				if let Some(child) = child {
					bytes += child.bytes();
					children.push(child);
				}
			}
	
			children.sort_unstable_by(|n1, n2| {n1.bytes.cmp(&n2.bytes)});
		}

		Some(
			Self {
				name: name.unwrap_or_else(|| String::from("-")),
				path_prefix: pre_path,
				bytes: bytes,
				children,
				big_rect: rect,
				small_rect, 
				color: random_col(if path.is_dir() {1.0} else {0.15}),
				hovered: false,
				is_leaf: true,
			}
		)
	}

	pub fn get_full_path(&self) -> String {
		let mut path = self.path_prefix.clone();
		path.push_str(&self.name);
		return path;
	}

	pub fn draw(&self) {
		if self.is_leaf {
			
			let mut half_rect_size = vec2(self.big_rect.w, self.big_rect.h*0.5);
			let margin = half_rect_size.min_element() * 0.1;
			half_rect_size -= 2.0*margin;

			draw_rectangle(self.big_rect.x, self.big_rect.y, self.big_rect.w, self.big_rect.h, self.color);

			let upper_text_dim = measure_text(&self.name, None, 16, 1.0);
			let lower_text_dim = measure_text(&bytes_to_text(self.bytes), None, 16, 1.0);

			let upper_text_size = vec2(upper_text_dim.width, upper_text_dim.height);
			let lower_text_size = vec2(lower_text_dim.width, lower_text_dim.height);

			let upper_text_max_scale = (half_rect_size / upper_text_size).min_element();
			let lower_text_max_scale = (half_rect_size / lower_text_size).min_element();

			let scale = upper_text_max_scale.min(lower_text_max_scale);

			draw_text_ex(
				&self.name, 
				self.big_rect.center().x - upper_text_dim.width * 0.5 * scale, 
				self.big_rect.center().y - margin - (upper_text_dim.height - upper_text_dim.offset_y)*scale, 
				TextParams { 
					font: None,
					font_size: 16, 
					font_scale: scale, 
					font_scale_aspect: 1.0, 
					rotation: 0.0, 
					color: WHITE,
				},
			);

			if self.hovered {
				draw_text_ex(
					&bytes_to_text(self.bytes), 
					self.big_rect.center().x - lower_text_dim.width * 0.5 * scale, 
					self.big_rect.center().y + margin + (lower_text_dim.offset_y)*scale, 
					TextParams { 
						font: None,
						font_size: 16,
						font_scale: scale, 
						font_scale_aspect: 1.0, 
						rotation: 0.0, 
						color: WHITE,
					},
				);
			}
		}
		else {
			let mut color = self.color.clone();
			color.r *= 0.7;
			color.g *= 0.7;
			color.b *= 0.7;
			draw_rectangle(self.big_rect.x, self.big_rect.y, self.big_rect.w, self.big_rect.h, color);

			for child in &self.children {
				child.draw();
			}
		}
	}

	// (tooltip, collapse parent)
	pub fn handle_mouse(&mut self, pos: Vec2, clicked_l: bool, clicked_r: bool) -> (Option<String>, bool) {
		let mut tooltip = None;

		if self.is_leaf {
			self.hovered = self.big_rect.contains(pos);
			if self.hovered {
				tooltip = Some(self.get_full_path());
			}

			if self.hovered && clicked_l && self.children.len() != 0 {
				self.is_leaf = false;				
				Self::place_children(&mut self.children, self.small_rect);
			}
			else if self.hovered && clicked_r {
				return (tooltip, true);
			}
		}
		else {
			self.hovered = false;

			let mut should_collapse = false;
			for child in &mut self.children {
				let resp = child.handle_mouse(pos, clicked_l, clicked_r);
				should_collapse |= resp.1;
				if resp.0.is_some() {
					tooltip = resp.0;
				}
			}
			if should_collapse {
				self.collapse_recursive();
			}

			if self.big_rect.contains(pos) && !self.small_rect.contains(pos) {
				tooltip = Some(self.get_full_path())
			}
		}		

		(tooltip, false)
	}

	pub fn collapse_recursive(&mut self) {
		self.is_leaf = true;

		for child in &mut self.children {
			child.collapse_recursive();
		}
	}

	fn place_children(slice: &mut [Node], rect: Rect) {
		// end condition - give all space if only one node is left
		if slice.len() == 1 {
			slice[0].big_rect = rect;
			slice[0].small_rect = rect.clone();
			shrink_rect_margin(&mut slice[0].small_rect, 0.05);
			return;
		}

		let mut size_sum = 0;

		for node in slice.iter() {
			size_sum += node.bytes;
		}

		let mut half_sum = 0;
		let mut split_index = 0;

		// find the index in slice, where [0, split_index) and [split_index, length) are as evenly split as possible
		for node in slice.iter() {
			half_sum += node.bytes;
			split_index += 1;
			if half_sum > size_sum/2 {
				break;
			}
		}

		// correct the split_index if overshoot
		if ((size_sum/2) as i128 - half_sum as i128).abs() >=
		   ((size_sum/2) as i128 - (half_sum as i128 - slice[split_index - 1].bytes as i128)).abs() {
			split_index -= 1;
			half_sum -= slice[split_index].bytes;
		}

		// shouldn't ever happen, but if so this prevents infinite recursion
		if split_index == 0 {
			split_index = 1;
			half_sum = slice[0].bytes;
		}
		if split_index == slice.len() {
			split_index = slice.len() - 1;
			half_sum = size_sum - slice[slice.len() - 1].bytes;
		}

		// split the rectangle
		let proportion = half_sum as f32 / size_sum as f32;
		let mut rect1 = rect.clone();
		let mut rect2 = rect.clone();

		if rect.w > rect.h || rect.w == rect.h && rand::gen_range(0, 2) == 0 {
			rect1.w *= proportion;
			rect2.w *= 1.0 - proportion;
			rect2.x += rect1.w;
		}
		else {
			rect1.h *= proportion;
			rect2.h *= 1.0 - proportion;
			rect2.y += rect1.h;
		}
		
		// divide further
		Self::place_children(&mut slice[..split_index], rect1);
		Self::place_children(&mut slice[split_index..], rect2);
	}

	pub fn bytes(&self) -> u64 {
		self.bytes
	}
}