use std::ops::{Add, Mul};
use std::path::Path;
use std::{io, fs};

use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub fn lerp<T>(a: T, b: T, t: f32) -> T 
where 
	f32 : Mul<T, Output = T>,
	T : Add<Output = T>,
{
	t * b + (1.0 - t) * a
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

pub fn random_col() -> Color {
	col_from_hsv(gen_range(0.0, 1.0), gen_range(0.4, 1.0), 1.0)
}

pub fn dir_size(path: &str) -> io::Result<u64> {
    fn dir_size(path: &Path) -> io::Result<u64> {

		let mut total_size = 0;

		for entry in fs::read_dir(path)? {
			let entry = entry?;
			let metadata = entry.metadata()?;
	
			total_size += match metadata {
				data if data.is_dir() && entry.file_name() == "System Volume Information" => 0,
				data if data.is_dir() => dir_size(&entry.path())?,
				data => data.len(),
			};
		}
		
		Ok(total_size)
    }

    dir_size(Path::new(path))
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
	let display_number = bytes as f32 / 1000f32.powf(order as f32);
	let dn_digits = if display_number == 0.0 { 1 } else { display_number.log10() as usize + 1 };
	return format!("{:.2$}{}B", display_number, units[order], (3 - dn_digits.min(3)));
}