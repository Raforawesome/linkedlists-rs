mod safe_linked_list;
mod alloc;

pub use safe_linked_list::{ LinkedList, LinkedNode };

#[cfg(test)]
mod tests {
	use crate::{ LinkedList, LinkedNode };

    #[test]
    fn it_works() {
		let _list: LinkedList<i32> = LinkedList::new(LinkedNode::head(2));
    }

	#[test]
	fn index_test() {
		let list: LinkedList<i32> = LinkedList::new(LinkedNode::head(2));
		let index: Option<usize> = list.index(2);
		dbg!(index);
	}
}
