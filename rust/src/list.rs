#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>
}

impl ListNode {
	#[inline]
	pub(crate) fn new(val: i32) -> Self {
		ListNode {
			next: None,
			val
		}
	}
	pub fn from_vec(array: Vec<i32>) -> Option<Box<Self>> {
		let mut head: Option<Box<Self>> = None;
		for value in array.into_iter().rev() {
			let mut node = Box::new(Self::new(value));
			node.next = head;
			head = Some(node);
		}
		head
	}

}