mod controls;
mod helper;
mod node;

use macroquad::prelude::*;
use node::Node;

#[macroquad::main("fsexpl")]
async fn main() {

	let mut controls = controls::Controls::new();
	
	let mut root = Node::root('D');

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
* recursive -> queue / while / iterative
* text placement
* borders, show path
* ux: empty -> choose path / show all
* asynchrony
*/