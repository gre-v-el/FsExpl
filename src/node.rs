use std::{fs::read_dir, path::{PathBuf, Path}};

use macroquad::prelude::*;

use crate::helper::{random_col, dir_size, bytes_to_text, shrink_rect_margin};

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
	pub fn new(path: &Path, rect: Rect) -> (Self, Vec<PathBuf>){
		let (bytes, denied) = dir_size(path);

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

		(Self { 
			name: name.unwrap_or_else(|| String::new()),
			path_prefix: pre_path,
			bytes: bytes,
			children: Vec::new(),
			big_rect: rect,
			small_rect, 
			color: random_col(if path.is_dir() {1.0} else {0.15}),
			hovered: false,
			is_leaf: true,
		}, denied)
	}

	pub fn draw(&self) {
		if self.children.len() == 0 {			
			
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

	pub fn handle_mouse(&mut self, pos: Vec2, clicked: bool) {
		if self.children.len() == 0 {
			self.hovered = self.big_rect.contains(pos);
			if self.hovered && clicked{
				println!("{:?}", self.split());
			}
		}
		else {
			self.hovered = false;
			for child in &mut self.children {
				child.handle_mouse(pos, clicked);
			}
		}		
	}

	fn split(&mut self) -> Vec<PathBuf> {
		let mut denied = Vec::new();

		let mut full_path = self.path_prefix.to_owned();
		full_path.push_str(&self.name);
		let full_path = PathBuf::from(full_path);

		let mut path_prefix = self.path_prefix.clone();
		path_prefix.push_str(&self.name);
		path_prefix.push('/');

		let iter = match read_dir(&full_path) {
			Ok(i) => i,
			Err(_) => {
				denied.push(full_path);
				return denied;
			},
		};

		for dir in iter {
			let dir = match dir {
				Ok(d) => d,
				Err(_) => {
					denied.push(full_path);
					return denied;
				},
			};

			let metadata = match dir.metadata() {
				Ok(m) => m,
				Err(_) => {
					denied.push(dir.path());
					return denied;
				},
			};
			
			let size = if metadata.is_dir() {
				let (size, errors) = dir_size(&dir.path());
				denied.extend(errors);
				size
			} else {
				metadata.len()
			};
			
			let node = Node {
				path_prefix: path_prefix.clone(),
				name: dir.file_name().into_string().unwrap(),
				bytes: size,
				children: Vec::new(),
				big_rect: Rect::new(1.0, 1.0, 1.0, 1.0),
				small_rect: Rect::new(0.9, 0.9, 0.8, 0.8),
				color: random_col(if metadata.is_dir() { 1.0 } else { 0.3 }),
				hovered: false,
				is_leaf: true,
			};

			self.children.push(node);
		}

		self.children.sort_unstable_by(|n1, n2| {n1.bytes.cmp(&n2.bytes)});

		Self::place_children(&mut self.children, self.small_rect);

		denied
	}

	fn place_children(slice: &mut [Node], rect: Rect) {
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

		for node in slice.iter() {
			half_sum += node.bytes;
			split_index += 1;
			if half_sum > size_sum/2 {
				break;
			}
		}

		if ((size_sum/2) as i128 - half_sum as i128).abs() >=
		   ((size_sum/2) as i128 - (half_sum as i128 - slice[split_index - 1].bytes as i128)).abs() {
			split_index -= 1;
			half_sum -= slice[split_index].bytes;
		}

		if split_index == 0 {
			split_index = 1;
			half_sum = slice[0].bytes;
		}
		if split_index == slice.len() {
			split_index = slice.len() - 1;
			half_sum = size_sum - slice[slice.len() - 1].bytes;
		}

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
		
		Self::place_children(&mut slice[..split_index], rect1);
		Self::place_children(&mut slice[split_index..], rect2);
	}
}