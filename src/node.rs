use std::fs::read_dir;

use macroquad::{prelude::*, rand::ChooseRandom};

use crate::helper::{random_col, dir_size, draw_centered_text, bytes_to_text};

#[derive(Debug)]
pub struct Node {
	path_prefix: String,
	name: String,
	bytes: u64,
	children: Vec<Node>,
	rect: Rect,
	color: Color,
	hovered: bool,
}

impl Node {
	pub fn root(drive: char) -> Self{
		// let name = format!("{drive}:/");
		let name = "D:/pliki/".to_owned();
		let bytes = match dir_size(&name) {
			Ok(v) => v,
			Err(e) => {
				println!("{e:?}");
				panic!();
			},
		};

		Self { 
			path_prefix: String::new(),
			name,
			bytes,
			children: Vec::new(),
			rect: Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 },
			color: random_col(),
			hovered: false,
		}
	}

	pub fn draw(&self) {
		if self.children.len() == 0 {
			draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);

			draw_centered_text(&self.name, self.rect.w/5.0, self.rect.center());

			if self.hovered {
				draw_centered_text(&bytes_to_text(self.bytes), self.rect.w/7.0, self.rect.center() + vec2(0.0, 0.1));
			}
		}
		else {
			for child in &self.children {
				child.draw();
			}
		}
	}

	pub fn handle_mouse(&mut self, pos: Vec2, clicked: bool) {
		if self.children.len() == 0 {
			self.hovered = self.rect.contains(pos);
			if self.hovered && clicked{
				self.split();
			}
		}
		else {
			self.hovered = false;
			for child in &mut self.children {
				child.handle_mouse(pos, clicked);
			}
		}		
	}

	fn split(&mut self) {
		let mut full_path = self.path_prefix.to_owned();
		full_path.push_str(&self.name);

		let mut path_prefix = self.path_prefix.clone();
		path_prefix.push_str(&self.name);
		path_prefix.push_str("/");

		for dir in read_dir(&full_path).unwrap() {
			let dir = dir.unwrap();
			if dir.file_name() == "System Volume Information" { continue; }
			let metadata = dir.metadata().unwrap();
			let size = if metadata.is_dir() {
				dir_size(dir.path().to_str().unwrap()).unwrap()
			} else {
				metadata.len()
			};
			
			let node = Node {
				path_prefix: path_prefix.clone(),
				name: dir.file_name().into_string().unwrap(),
				bytes: size,
				children: Vec::new(),
				rect: Rect::new(1.0, 1.0, 1.0, 1.0),
				color: random_col(),
				hovered: false,
			};

			self.children.push(node);
		}

		Self::place_children(&mut self.children, self.rect);
	}

	fn place_children(slice: &mut [Node], rect: Rect) {
		if slice.len() == 1 {
			slice[0].rect = rect;
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

		if ((size_sum/2) as i128 - half_sum as i128).abs() > 
		   ((size_sum/2) as i128 - (half_sum as i128 - slice[split_index - 1].bytes as i128)).abs() {
			split_index -= 1;
			half_sum -= slice[split_index].bytes;
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