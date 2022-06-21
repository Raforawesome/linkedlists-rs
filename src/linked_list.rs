#![allow(unused, dead_code)]

use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct LinkedNode<T>
where T: PartialEq
{
	data: T,
	next: Option<Box<LinkedNode<T>>>
}

impl<T> PartialEq for LinkedNode<T>
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> LinkedNode<T>
where T: PartialEq
{
	pub fn head(data: T) -> Self {
		Self { data, next: None }
	}

	pub fn get(&self) -> &T {
		&self.data
	}

	pub fn get_mut(&mut self) -> &mut T {
		&mut self.data
	}

	pub fn set_next(&mut self, new: Option<Box<LinkedNode<T>>>) {
		self.next = new;
	}
}


pub struct LinkedList<T>
where T: PartialEq
{
	head: LinkedNode<T>,
	size: usize
}

impl<T> LinkedList<T>
where T: PartialEq
{
	pub fn new(head: LinkedNode<T>) -> Self {
		Self { head, size: 1_usize }
	}

	pub fn index(&self, val: T) -> Option<usize> {
		let mut count: usize = 0;
		let mut current: Box<&LinkedNode<T>> = Box::new(&self.head);

		loop {
			let borrow: &LinkedNode<T> = *current.clone();
			if *borrow.get() == val {
				return Some(count)
			}
			current = 
			count += 1;
		}
		todo!()
	}
}