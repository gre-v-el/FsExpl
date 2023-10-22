use std::{thread, sync::mpsc::{channel, Receiver}, path::{Path, PathBuf}};

use egui_macroquad::macroquad::prelude::Rect;

use crate::node::Node;

#[derive(Debug)]
pub enum Message {
	Denied(PathBuf),
	Progress(u64), // total bytes
	Finished(Option<Node>), // Some if successful, None if invalid
}

pub enum Tree {
	Empty,
	Processing(u64, u64, Receiver<Message>), // bytes, files cummulative, message receiver
	Ready(Node, u64, u64), // tree, bytes, files
}

impl Tree {
	pub fn scan(&mut self, path: &Path) {
		if self.is_processing() {
			return;
		}

		let (mut sender, receiver) = channel::<Message>();
		
		let path_buf = path.to_owned();
		thread::spawn(move || {
			let mut bytes = 0;
			let mut counter = 0;
			let node = Node::new(&path_buf, Rect::new(0.0, 0.0, 1.0, 1.0), &mut sender, &mut bytes, &mut counter);

			sender.send(Message::Finished(node)).unwrap();
		});

		*self = Self::Processing(0, 0, receiver);
	}

	// returns true if the tree finished loading
	pub fn process(&mut self, denied: &mut Vec<PathBuf>) -> bool {
		let mut finished_node = None;

		if let Tree::Processing(bytes, files, receiver) = self {
			while let Ok(m) = receiver.try_recv() {
				match m {
					Message::Progress(new_bytes) => { *bytes = new_bytes; *files += 100;},
					Message::Denied(path) => denied.push(path),
					Message::Finished(node) => finished_node = Some((node, *bytes, *files)),
				}
			}
		}

		if let Some(node) = finished_node {
			if let (Some(node), bytes, files) = node {
				*self = Tree::Ready(node, bytes, files);
				return true;
			}
			else {
				*self = Tree::Empty;
			}
		}

		false
	}

	pub fn _is_empty(&self) -> bool {
		match self {
			&Self::Empty => true,
			&Self::Processing(_, _, _) => false,
			&Self::Ready(_, _, _) => false,
		}
	}

	pub fn is_processing(&self) -> bool {
		match self {
			&Self::Empty => false,
			&Self::Processing(_, _, _) => true,
			&Self::Ready(_, _, _) => false,
		}
	}

	pub fn _is_ready(&self) -> bool {
		match self {
			&Self::Empty => false,
			&Self::Processing(_, _, _) => false,
			&Self::Ready(_, _, _) => true,
		}
	}
}