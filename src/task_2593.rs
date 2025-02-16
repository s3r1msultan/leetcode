/*You are given an array nums consisting of positive integers.

Starting with score = 0, apply the following algorithm:

Choose the smallest integer of the array that is not marked. If there is a tie, choose the one with the smallest index.
Add the value of the chosen integer to score.
Mark the chosen element and its two adjacent elements if they exist.
Repeat until all the array elements are marked.

Return the score you get after applying the above algorithm.



Example 1:

Input: nums = [2,1,3,4,5,2]
Output: 7
Explanation: We mark the elements as follows:
- 1 is the smallest unmarked element, so we mark it and its two adjacent elements: [2,1,3,4,5,2].
- 2 is the smallest unmarked element, so we mark it and its left adjacent element: [2,1,3,4,5,2].
- 4 is the only remaining unmarked element, so we mark it: [2,1,3,4,5,2].
Our score is 1 + 2 + 4 = 7.

Example 2:

Input: nums = [2,3,5,1,3,2]
Output: 5
Explanation: We mark the elements as follows:
- 1 is the smallest unmarked element, so we mark it and its two adjacent elements: [2,3,5,1,3,2].
- 2 is the smallest unmarked element, since there are two of them, we choose the left-most one, so we mark the one at index 0 and its right adjacent element: [2,3,5,1,3,2].
- 2 is the only remaining unmarked element, so we mark it: [2,3,5,1,3,2].
Our score is 1 + 2 + 2 = 5.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 106

*/

pub fn find_score(nums: Vec<i32>) -> i64 {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};
    let n = nums.len();
    let mut min_heap = BinaryHeap::new();
    let mut set = HashSet::new();
    let mut max = 0i64;

    for (i, &num) in nums.iter().enumerate() {
        min_heap.push(Reverse((num, i)));
    }

    while let Some(Reverse((num, i))) = min_heap.pop() {
        if set.contains(&i) {
            continue;
        }
        set.insert(i);
        if i + 1 < n {
            set.insert(i + 1);
        }
        if i as i32 - 1 < n as i32 {
            set.insert(i - 1);
        }
        max += num as i64;
    }

    max
}