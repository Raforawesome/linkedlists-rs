#[derive(Debug, Clone)]
pub struct LinkedNode<T> {
	data: T,
	next: Option<Box<LinkedNode<T>>>
}

impl<T> LinkedNode<T> {
	pub fn raw(data: T) -> Self {
		Self { data, next: None }
	}

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn next(&self) -> Option<&Box<LinkedNode<T>>> {
        self.next.as_ref()
    }

    pub fn set_next(&mut self, next: Option<Box<LinkedNode<T>>>) {
        self.next = next;
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<LinkedNode<T>>> {
        &mut self.next
    }
}


#[derive(Debug, Clone)]
pub struct LinkedList<T> {
	head: LinkedNode<T>,
	tail: LinkedNode<T>
}