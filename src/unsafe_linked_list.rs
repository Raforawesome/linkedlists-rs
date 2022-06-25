#![allow(unused, dead_code)]
use crate::alloc;

#[derive(Debug, Clone)]
pub struct LinkedNode<T>
where T: Clone
{
	data: T,
	next: *mut LinkedNode<T>,
	has_next: bool,
}

impl<T> LinkedNode<T>
where T: Clone
{
	pub fn new(data: T) -> Self {
		Self {
			data,
			next: alloc::null_ptr::<LinkedNode<T>>(),
			has_next: false,
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
		self.has_next = true;
	}
}

impl<T> Drop for LinkedNode<T>
where T: Clone
{
    fn drop(&mut self) {
		alloc::raw_dealloc(self.next);
    }
}


struct LinkedList<T>
where T: Clone
{
	head: LinkedNode<T>,
	tail: *mut LinkedNode<T>,
	size: usize,
}

impl<T> LinkedList<T>
where T: Clone
{
	pub fn new(head: LinkedNode<T>) -> Self {
		let ptr: *mut LinkedNode<T> = alloc::null_ptr();
		Self {
			head,
			tail: ptr,
			size: 1,
		}
	}
}