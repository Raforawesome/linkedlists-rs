#![allow(unused, dead_code)]

use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct LinkedNode<T>
where T: PartialEq + Clone
{
	data: T,
	next: Option<Box<LinkedNode<T>>>
}

impl<T> PartialEq for LinkedNode<T>
where T: PartialEq + Clone
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> LinkedNode<T>
where T: PartialEq + Clone
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


#[derive(Debug, Clone)]
pub struct LinkedList<T>
where T: PartialEq + Clone
{
	head: Box<LinkedNode<T>>,
	size: usize
}

impl<T> LinkedList<T>
where T: PartialEq + Clone
{
	pub fn new(head: LinkedNode<T>) -> Self {
		Self { head: Box::new(head), size: 1_usize }
	}

	pub fn index(&self, val: T) -> Option<usize> {
		let mut count: usize = 0;
		let mut current: Box<LinkedNode<T>> = self.head.clone();

		loop {
			let borrow: LinkedNode<T> = *current.clone();
			if *borrow.get() == val {
				return Some(count)
			}

			let next: Option<Box<LinkedNode<T>>> = current.next;
			if let Some(node) = next {
				current = node;
			} else {
				return None
			}
			count += 1;
		}
	}
}