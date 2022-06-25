#![allow(unused, dead_code)]
use crate::alloc;

#[derive(Debug, Clone)]
pub struct LinkedNode<T> {
	data: T,
	next: *mut LinkedNode<T>
}

impl<T> LinkedNode<T> {
	pub fn new(data: T) -> Self {
		Self {
			data,
			next: alloc::null_ptr::<LinkedNode<T>>()
		}
	}

	pub fn get(&self) -> &T {
		&self.data
	}

	pub fn set_data(&mut self, new: T) {
		self.data = new;
	}

	pub fn set_next(&mut self, node: LinkedNode<T>) {
		let ptr: *mut LinkedNode<T> = alloc::raw_alloc();
		unsafe { *ptr = node };
		self.next = ptr;
	}
}