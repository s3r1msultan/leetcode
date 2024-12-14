/*You are given a 0-indexed integer array nums. A subarray of nums is called continuous if:

Let i, i + 1, ..., j be the indices in the subarray. Then, for each pair of indices i <= i1, i2 <= j, 0 <= |nums[i1] - nums[i2]| <= 2.

Return the total number of continuous subarrays.

A subarray is a contiguous non-empty sequence of elements within an array.



Example 1:

Input: nums = [5,4,2,4]
Output: 8
Explanation:
Continuous subarray of size 1: [5], [4], [2], [4].
Continuous subarray of size 2: [5,4], [4,2], [2,4].
Continuous subarray of size 3: [4,2,4].
Thereare no subarrys of size 4.
Total continuous subarrays = 4 + 3 + 1 = 8.
It can be shown that there are no more continuous subarrays.



Example 2:

Input: nums = [1,2,3]
Output: 6
Explanation:
Continuous subarray of size 1: [1], [2], [3].
Continuous subarray of size 2: [1,2], [2,3].
Continuous subarray of size 3: [1,2,3].
Total continuous subarrays = 3 + 2 + 1 = 6.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 109

*/

pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let mut count = 0i64;
    let mut left = 0;
    let mut min_heap = BinaryHeap::new();
    let mut max_heap = BinaryHeap::new();

    for right in 0..nums.len() {
        min_heap.push(Reverse((nums[right], right)));
        max_heap.push((nums[right], right));
        while left < right {
            let min = min_heap.peek().unwrap().0.1;
            let max = max_heap.peek().unwrap().1;
            if nums[max] - nums[min] > 2 {
                left += 1;

                while !min_heap.is_empty() && left > min_heap.peek().unwrap().0.1 {
                    min_heap.pop();
                }

                while !max_heap.is_empty() && left > max_heap.peek().unwrap().1 {
                    max_heap.pop();
                }
            } else {
                break;
            }
        }
        count += (right - left + 1) as i64;
    }
    count
}