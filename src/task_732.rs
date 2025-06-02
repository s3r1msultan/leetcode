struct MyCalendarThree {
    root: SegmentTree,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            root: SegmentTree::new(0, 1_000_000_000)
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.root.update(start_time, end_time - 1, 1)
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */


struct SegmentTree {
    left: Option<Box<SegmentTree>>,
    right: Option<Box<SegmentTree>>,
    max: i32,
    add: i32,
    start: i32,
    end: i32,
}


impl SegmentTree {
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            left: None,
            right: None,
            max: 0,
            add: 0,
            start,
            end,
        }
    }

    pub fn update(&mut self, tree_left: i32, tree_right: i32, val: i32) -> i32 {
        if tree_left > self.end || tree_right < self.start {
            return 0;
        }

        if tree_left <= self.start && self.end <= tree_right {
            self.max += val;
            self.add += val;
            return self.max;
        }

        if self.left.is_none() || self.right.is_none() {
            let mid = self.start + (self.end - self.start) / 2;
            self.left = Some(Box::new(SegmentTree::new(self.start, mid)));
            self.right = Some(Box::new(SegmentTree::new(mid + 1, self.end)));
        }

        if self.add != 0 {
            self.left.as_mut().unwrap().max += self.add;
            self.left.as_mut().unwrap().add += self.add;

            self.right.as_mut().unwrap().max += self.add;
            self.right.as_mut().unwrap().add += self.add;

            self.add = 0;
        }


        self.left.as_mut().unwrap().update(tree_left, tree_right, val);
        self.right.as_mut().unwrap().update(tree_left, tree_right, val);
        self.max = self.left.as_ref().unwrap().max.max(self.right.as_ref().unwrap().max);
        self.max
    }
}