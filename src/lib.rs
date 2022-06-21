mod linked_list;
mod alloc;

pub use linked_list::{ LinkedList, LinkedNode };

#[cfg(test)]
mod tests {
	use crate::{ LinkedList, LinkedNode };

    #[test]
    fn it_works() {
		let _list: LinkedList<i32> = LinkedList::new(LinkedNode::head(2));
    }
}
