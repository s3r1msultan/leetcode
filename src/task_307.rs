/*Given an integer array nums, handle multiple queries of the following types:

Update the value of an element in nums.
Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.

Implement the NumArray class:

NumArray(int[] nums) Initializes the object with the integer array nums.
void update(int index, int val) Updates the value of nums[index] to be val.
int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).



Example 1:

Input
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
Output
[null, 9, null, 8]

Explanation
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // return 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1, 2, 5]
numArray.sumRange(0, 2); // return 1 + 2 + 5 = 8



Constraints:

1 <= nums.length <= 3 * 104
-100 <= nums[i] <= 100
0 <= index < nums.length
-100 <= val <= 100
0 <= left <= right < nums.length
At most 3 * 104 calls will be made to update and sumRange.

*/

struct NumArray {
    n: usize,
    tree: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let tree = vec![0; n * 4];
        let mut segment_tree = NumArray { n, tree };
        segment_tree.build(&nums, 0, 0, n - 1);
        segment_tree
    }

    fn build(&mut self, arr: &Vec<i32>, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = start + (end - start) / 2;
            let left_child = node * 2 + 1;
            let right_child = node * 2 + 2;

            self.build(arr, left_child, start, mid);
            self.build(arr, right_child, mid + 1, end);

            self.tree[node] = self.tree[left_child] + self.tree[right_child];
        }
    }

    fn update(&mut self, index: usize, val: i32) {
        self.update_recursive(0, 0, self.n - 1, index, val);
    }

    fn update_recursive(&mut self, node: usize, start: usize, end: usize, index: usize, val: i32) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = start + (end - start) / 2;
            let left_child = node * 2 + 1;
            let right_child = node * 2 + 2;

            if index <= mid {
                self.update_recursive(left_child, start, mid, index, val);
            } else {
                self.update_recursive(right_child, mid + 1, end, index, val);
            }

            self.tree[node] = self.tree[left_child] + self.tree[right_child];
        }
    }

    fn sum_range(&self, left: usize, right: usize) -> i32 {
        self.sum_range_recursive(0, 0, self.n - 1, left, right)
    }


    fn sum_range_recursive(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if start > right || end < left {
            0
        } else if start >= left && end <= right {
            self.tree[node]
        } else {
            let mid = start + (end - start) / 2;
            let left_child = node * 2 + 1;
            let right_child = node * 2 + 2;

            let sum_left = self.sum_range_recursive(left_child, start, mid, left, right);
            let sum_right = self.sum_range_recursive(right_child, mid + 1, end, left, right);

            sum_left + sum_right
        }
    }
}
