/*Given an empty set of intervals, implement a data structure that can:

Add an interval to the set of intervals.
Count the number of integers that are present in at least one interval.

Implement the CountIntervals class:

CountIntervals() Initializes the object with an empty set of intervals.
void add(int left, int right) Adds the interval [left, right] to the set of intervals.
int count() Returns the number of integers that are present in at least one interval.

Note that an interval [left, right] denotes all the integers x where left <= x <= right.



Example 1:

Input
["CountIntervals", "add", "add", "count", "add", "count"]
[[], [2, 3], [7, 10], [], [5, 8], []]
Output
[null, null, null, 6, null, 8]

Explanation
CountIntervals countIntervals = new CountIntervals(); // initialize the object with an empty set of intervals.
countIntervals.add(2, 3);  // add [2, 3] to the set of intervals.
countIntervals.add(7, 10); // add [7, 10] to the set of intervals.
countIntervals.count();    // return 6
// the integers 2 and 3 are present in the interval [2, 3].
// the integers 7, 8, 9, and 10 are present in the interval [7, 10].
countIntervals.add(5, 8);  // add [5, 8] to the set of intervals.
countIntervals.count();    // return 8
// the integers 2 and 3 are present in the interval [2, 3].
// the integers 5 and 6 are present in the interval [5, 8].
// the integers 7 and 8 are present in the intervals [5, 8] and [7, 10].
// the integers 9 and 10 are present in the interval [7, 10].



Constraints:

1 <= left <= right <= 109
At most 105 calls in total will be made to add and count.
At least one call will be made to count.

*/

struct CountIntervals {
    root: SegmentTree,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    fn new() -> Self {
        Self {
            root: SegmentTree::new(0, 1_000_000_000)
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        self.root.update(left, right)
    }

    fn count(&self) -> i32 {
        self.root.count
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */
#[derive(Debug)]
struct SegmentTree {
    left: Option<Box<SegmentTree>>,
    right: Option<Box<SegmentTree>>,
    start: i32,
    end: i32,
    count: i32,
    is_covered: bool,
}

impl SegmentTree {
    fn new(start: i32, end: i32) -> Self {
        Self {
            left: None,
            right: None,
            start,
            end,
            count: 0,
            is_covered: false,
        }
    }

    fn update(&mut self, left: i32, right: i32) {
        if self.start > right || self.end < left || self.is_covered {
            return;
        }

        if left <= self.start && self.end <= right {
            self.count = self.end - self.start + 1;
            self.is_covered = true;
            self.left = None;
            self.right = None;
            return;
        }

        let mid = self.start + (self.end - self.start) / 2;
        if self.left.is_none() {
            self.left = Some(Box::new(SegmentTree::new(self.start, mid)));
        }
        if self.right.is_none() {
            self.right = Some(Box::new(SegmentTree::new(mid + 1, self.end)));
        }
        self.left.as_mut().unwrap().update(left, right);
        self.right.as_mut().unwrap().update(left, right);
        self.count = self.left.as_ref().unwrap().count + self.right.as_ref().unwrap().count;
    }
}