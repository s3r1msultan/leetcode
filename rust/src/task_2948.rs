/*You are given a 0-indexed array of positive integers nums and a positive integer limit.

In one operation, you can choose any two indices i and j and swap nums[i] and nums[j] if |nums[i] - nums[j]| <= limit.

Return the lexicographically smallest array that can be obtained by performing the operation any number of times.

An array a is lexicographically smaller than an array b if in the first position where a and b differ, array a has an element that is less than the corresponding element in b. For example, the array [2,10,3] is lexicographically smaller than the array [10,2,3] because they differ at index 0 and 2 < 10.



Example 1:

Input: nums = [1,5,3,9,8], limit = 2
Output: [1,3,5,8,9]
Explanation: Apply the operation 2 times:
- Swap nums[1] with nums[2]. The array becomes [1,3,5,9,8]
- Swap nums[3] with nums[4]. The array becomes [1,3,5,8,9]
We cannot obtain a lexicographically smaller array by applying any more operations.
Note that it may be possible to get the same result by doing different operations.

Example 2:

Input: nums = [1,7,6,18,2,1], limit = 3
Output: [1,6,7,18,1,2]
Explanation: Apply the operation 3 times:
- Swap nums[1] with nums[2]. The array becomes [1,6,7,18,2,1]
- Swap nums[0] with nums[4]. The array becomes [2,6,7,18,1,1]
- Swap nums[0] with nums[5]. The array becomes [1,6,7,18,1,2]
We cannot obtain a lexicographically smaller array by applying any more operations.

Example 3:

Input: nums = [1,7,28,19,10], limit = 3
Output: [1,7,28,19,10]
Explanation: [1,7,28,19,10] is the lexicographically smallest array we can obtain because we cannot apply the operation on any two indices.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 109
1 <= limit <= 109

*/
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct DS {
    representatives: Vec<usize>,
    ranks: Vec<i32>
}

impl DS {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            ranks: vec![1; n]
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if x == parent {
            return x;
        }

        let root = self.find(parent);
        self.representatives[x] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.ranks[root_a] > self.ranks[root_b] {
            self.representatives[root_b] = root_a;
        } else if self.ranks[root_b] > self.ranks[root_a] {
            self.representatives[root_a] = root_b;
        } else {
            self.representatives[root_b] = root_a;
            self.ranks[root_a] += 1;
        }
    }
}

pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let n = nums.len();
    if n == 0 {
        return vec![];
    }

    let mut pairs = nums.iter().enumerate().map(|(i, &num)| (num, i)).collect::<Vec<_>>();

    pairs.sort();

    let mut set = DS::new(n);

    for i in 1..n {
        if pairs[i].0 - pairs[i-1].0 <= limit {
            set.union(pairs[i].1, pairs[i-1].1);
        }
    }

    let mut m = HashMap::new();
    for (i, &c) in set.representatives.iter().enumerate() {
        m.entry(c)
            .and_modify(|x: &mut BinaryHeap<Reverse<i32>>| x.push(Reverse(nums[i])))
            .or_insert_with(|| {
                let mut heap = BinaryHeap::new();
                heap.push(Reverse(nums[i]));
                heap
            });
    }

    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(m.get_mut(&set.representatives[i]).unwrap().pop().unwrap().0);
    }
    ans
}