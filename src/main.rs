mod controls;
mod helper;
mod node;
mod tree;
mod state;
mod icon;

use egui_macroquad::macroquad::{self, prelude::*};
use state::State;

#[macroquad::main("fsexpl")]
async fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	let mut state = State::new(); 

	loop {
		state.frame();

		next_frame().await;
	}
}

/*
TODO:
* use a single vector for the tree, store a slice
* choose color palette
* sort by size/sort alphabetically/shuffle
* area scales - linear/square/logarithmic
* precompute per-frame calculations
*/