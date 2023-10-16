mod controls;
mod helper;
mod node;

use std::{env, path::Path};

use macroquad::prelude::*;
use node::Node;

#[macroquad::main("fsexpl")]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let mut controls = controls::Controls::new();
	
	let (mut root, denied) = Node::new(Path::new("D:/pliki/programowanie/"), Rect::new(0.0, 0.0, 1.0, 1.0));
	println!("{:?}", denied);

	loop {
		clear_background(BLACK);

		set_camera(controls.camera());
		root.draw();

		root.handle_mouse(*controls.mouse_world(), is_mouse_button_pressed(MouseButton::Left));
		controls.update();

		next_frame().await;
	}
}

/*
TODO:
* borders, show path
* ux: empty -> choose path / show all
* asynchrony
* list all folders without permissions
* sort by size/sort alphabetically/shuffle
* area scales - linear/square/logarithmic
* collapse with RMB
* precompute per-frame calculations
*/