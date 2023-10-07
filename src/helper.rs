use std::ops::{Add, Sub, Mul};
use std::path::{Path, PathBuf};
use std::fs;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub fn lerp<T, U>(a: T, b: T, t: U) -> T 
where 
	U : Mul<T, Output = T> + Copy,
	T : Add<Output = T> + Sub<Output = T> + Copy,
{
	t * b + a - t * a
}

pub fn col_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {

	let hue = hue + hue.floor();
	let hue = hue % 1.0;
	let saturation = saturation.clamp(0.0, 1.0);
	let value = value.clamp(0.0, 1.0);

    let h = (hue * 6.0) as i32;
    let f = hue * 6.0 - h as f32;
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - f * saturation);
    let t = value * (1.0 - (1.0 - f) * saturation);

    match h {
      0 => Color::new(value, t, p, 1.0),
      1 => Color::new(q, value, p, 1.0),
      2 => Color::new(p, value, t, 1.0),
      3 => Color::new(p, q, value, 1.0),
      4 => Color::new(t, p, value, 1.0),
      5 => Color::new(value, p, q, 1.0),
	  _ => unreachable!()
    }
}

pub fn random_col(brightness: f32) -> Color {
	col_from_hsv(gen_range(0.0, 1.0), gen_range(0.4, 1.0), brightness)
}

pub fn dir_size(path: &Path) -> (u64, Vec<PathBuf>) {
	
	fn dir_size(path: &Path, denied: &mut Vec<PathBuf>) -> u64 {
		let mut total_size = 0;


		let iterator = match fs::read_dir(path) {
			Ok(i) => i,
			Err(_) => {
				denied.push(path.to_path_buf());
				return 0;
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

			let metadata = match entry.metadata() {
				Ok(m) => m,
				Err(_) => {
					denied.push(entry.path());
					continue;
				}
			};
			
			if metadata.is_dir() {
				total_size += dir_size(&entry.path(), denied);
			}
			else {
				total_size += metadata.len();
			}
		}
		
		total_size
    }
	
	let mut denied = Vec::new();
    (dir_size(Path::new(path), &mut denied), denied)
}

pub fn draw_centered_text(text: &str, size: f32, pos: Vec2) {
	let text_size = measure_text(text, None, 16, 1.0/16.0*size);
	draw_text_ex(
		text, 
		pos.x - text_size.width/2.0, 
		pos.y, 
		TextParams { 
			font: None, 
			font_size: 16, 
			font_scale: 1.0/16.0*size, 
			font_scale_aspect: 1.0, 
			rotation: 0.0, 
			color: WHITE,
		}
	);
}

pub fn bytes_to_text(bytes: u64) -> String {
	let units = ["", "K", "M", "G", "T", "P", "E"];
	let order = if bytes == 0 { 0 } else { ((bytes as f32).log2() * 0.1) as usize };
	let order = order.min(units.len() - 1);
	let display_number = bytes as f32 / 1024u32.pow(order as u32) as f32;
	let dn_digits = if display_number == 0.0 { 1 } else { display_number.log10() as usize + 1 };
	return format!("{:.2$}{}B", display_number, units[order], (3 - dn_digits.min(3)));
}