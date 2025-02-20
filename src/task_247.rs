/*Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.



Example 1:

Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]

Example 2:

Input: nums = [1], k = 1
Output: [1]



Constraints:

1 <= nums.length <= 105
-104 <= nums[i] <= 104
k is in the range [1, the number of unique elements in the array].
It is guaranteed that the answer is unique.



Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
*/

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for &num in &nums {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut max_heap = std::collections::BinaryHeap::new();
    for (val, count) in map {
        max_heap.push((count, val));
    }
    let mut result = vec![];
    for _ in 0..k {
        result.push(max_heap.pop().unwrap().1);
    }
    result
}