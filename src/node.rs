use std::{fs::read_dir, path::{PathBuf, Path}};

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
		let name = "C:/Windows/Logs/".to_owned();
		let bytes = dir_size(Path::new(&name));
		println!("{bytes:?}");

		Self { 
			path_prefix: String::new(),
			name,
			bytes: bytes.0,
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
				rect: Rect::new(1.0, 1.0, 1.0, 1.0),
				color: random_col(),
				hovered: false,
			};

			self.children.push(node);
		}

		self.children.sort_unstable_by(|n1, n2| {n1.bytes.cmp(&n2.bytes)});

		Self::place_children(&mut self.children, self.rect);

		denied
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