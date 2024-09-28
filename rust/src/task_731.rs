/*You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a triple booking.

A triple booking happens when three events have some non-empty intersection (i.e., some moment is common to all the three events.).

The event can be represented as a pair of integers start and end that represents a booking on the half-open interval [start, end), the range of real numbers x such that start <= x < end.

Implement the MyCalendarTwo class:

MyCalendarTwo() Initializes the calendar object.
boolean book(int start, int end) Returns true if the event can be added to the calendar successfully without causing a triple booking. Otherwise, return false and do not add the event to the calendar.



Example 1:

Input
["MyCalendarTwo", "book", "book", "book", "book", "book", "book"]
[[], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]]
Output
[null, true, true, true, false, true, true]

Explanation
MyCalendarTwo myCalendarTwo = new MyCalendarTwo();
myCalendarTwo.book(10, 20); // return True, The event can be booked.
myCalendarTwo.book(50, 60); // return True, The event can be booked.
myCalendarTwo.book(10, 40); // return True, The event can be double booked.
myCalendarTwo.book(5, 15);  // return False, The event cannot be booked, because it would result in a triple booking.
myCalendarTwo.book(5, 10); // return True, The event can be booked, as it does not use time 10 which is already double booked.
myCalendarTwo.book(25, 55); // return True, The event can be booked, as the time in [25, 40) will be double booked with the third event, the time [40, 50) will be single booked, and the time [50, 55) will be double booked with the second event.



Constraints:

0 <= start < end <= 109
At most 1000 calls will be made to book.

*/

struct MyCalendarTwo {
	booked: Vec<(i32, i32)>,
	overlaps: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
	fn new() -> Self {
		MyCalendarTwo {
			root: Box::new(Interval::new(0, 1_000_000_000))
		}
	}

	fn book(&mut self, start: i32, end: i32) -> bool {
		if self.dfs(&self.root, start, end) {
			return false;
		}
		self.update(&mut self.root, start, end);
		true
	}

	fn dfs(&mut self, current_node: &Box<Interval>, given_start: i32, given_end: i32) -> bool {
		let start = current_node.start;
		let end = current_node.end;
		if start >= given_end || end <= given_start {
			return false;
		}
		if current_node.count_of_booking >= 2 {
			return true;
		}

		let left_has_overlapping = if let Some(ref node) = current_node.left {
			self.dfs(node, given_start, given_end)
		} else {
			false
		};

		let right_has_overlapping = if let Some(ref node) = current_node.right {
			self.dfs(node, given_start, given_end)
		} else {
			false
		};

		left_has_overlapping || right_has_overlapping
	}

	fn update(&mut self, current_node: &mut Box<Interval>, given_start: i32, given_end: i32) {
		if current_node.start >= given_end || current_node.end <= given_start {
			return;
		}
		current_node.count_of_booking += 1;
		let mid = current_node.start + (current_node.end - current_node.start) / 2;
		if current_node.left.is_none() {
			current_node.left = Some(Box::new(Interval::new(current_node.start, mid)));
		}

		if current_node.right.is_none() {
			current_node.right = Some(Box::new(Interval::new(mid, current_node.end)));
		}

		self.update(current_node.left.as_mut().unwrap(), given_start, given_end);
		self.update(current_node.right.as_mut().unwrap(), given_start, given_end);
	}
}

struct Interval {
	count_of_booking: i32,
	left: Option<Box<Interval>>,
	right: Option<Box<Interval>>,
	start: i32,
	end: i32,
}

impl Interval {
	pub fn new(start: i32, end: i32) -> Self {
		Interval {
			count_of_booking: 0,
			left: None,
			right: None,
			start,
			end,
		}
	}
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */