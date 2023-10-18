// use std::{thread::{Thread, self}, sync::mpsc::channel, path::Path};

// use crate::node::Node;

// enum Message {
// 	Denied(String),
// 	Progress(u64), // bytes
// 	Finished(Node),
// }

// enum Tree {
// 	Empty,
// 	Processing,
// 	Ready(Node),
// }

// impl Tree {
// 	pub fn scan(&mut self, path: &Path) {
// 		let (sender, receiver) = channel::<Message>();
		
// 		thread::spawn(move || {
// 			let node = Some(Node::new(Path::new(path), Rect::new(0.0, 0.0, 1.0, 1.0), &mut denied).unwrap());

// 		});

// 		*self = Self::Processing;
// 	}
// }