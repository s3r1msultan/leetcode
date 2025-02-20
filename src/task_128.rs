/*Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

You must write an algorithm that runs in O(n) time.



Example 1:

Input: nums = [100,4,200,1,3,2]
Output: 4
Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore, its length is 4.

Example 2:

Input: nums = [0,3,7,2,5,8,4,6,0,1]
Output: 9



Constraints:

0 <= nums.length <= 105
-109 <= nums[i] <= 109

*/
use std::collections::HashMap;

struct DS {
    representatives: HashMap<i32, i32>,
    size: HashMap<i32, i32>,
}

impl DS {
    fn new(nums: &Vec<i32>) -> Self {
        let mut representatives = HashMap::new();
        let mut size = HashMap::new();
        for &num in nums {
            representatives.insert(num, num);
            size.insert(num, 1);
        }
        Self {
            representatives,
            size,
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        let parent = *self.representatives.get(&x).unwrap();
        if x == parent {
            return x;
        }
        let root = self.find(parent);
        self.representatives.insert(x, root);
        root
    }

    fn union(&mut self, a: i32, b: i32) {
        if self.representatives.get(&a).is_none() || self.representatives.get(&b).is_none() {
            return;
        }

        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        let size_a = self.size[&root_a];
        let size_b = self.size[&root_b];

        if size_a > size_b {
            self.representatives.insert(root_b, root_a);
            *self.size.entry(root_a).or_insert(0) += size_b;
        } else {
            self.representatives.insert(root_a, root_b);
            *self.size.entry(root_b).or_insert(0) += size_a;
        }
    }

    fn get_size(&mut self, x: i32) -> i32 {
        let root = self.find(x);
        self.size[&root]
    }
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set = DS::new(&nums);
    for &num in &nums {
        set.union(num, num + 1);
        set.union(num, num - 1);
    }

    let mut max = 0;
    for &num in &nums {
        max = max.max(set.get_size(num));
    }
    max
}