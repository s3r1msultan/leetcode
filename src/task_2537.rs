/*Given an integer array nums and an integer k, return the number of good subarrays of nums.

A subarray arr is good if there are at least k pairs of indices (i, j) such that i < j and arr[i] == arr[j].

A subarray is a contiguous non-empty sequence of elements within an array.



Example 1:

Input: nums = [1,1,1,1,1], k = 10
Output: 1
Explanation: The only good subarray is the array nums itself.

Example 2:

Input: nums = [3,1,4,3,2,2,4], k = 2
Output: 4
Explanation: There are 4 different good subarrays:
- [3,1,4,3,2,2] that has 2 pairs.
- [3,1,4,3,2,2,4] that has 3 pairs.
- [1,4,3,2,2,4] that has 2 pairs.
- [4,3,2,2,4] that has 2 pairs.



Constraints:

1 <= nums.length <= 105
1 <= nums[i], k <= 109

*/

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();

    let mut left = 0;
    let mut pairs = 0;
    let mut map = std::collections::HashMap::new();
    let mut result: i64 = 0;

    for right in 0..n {
        let entry = map.entry(nums[right]).or_insert(0);
        pairs += *entry;
        *entry += 1;

        while pairs >= k {
            result += (nums.len() - right) as i64;
            let count = map.get_mut(&nums[left]).unwrap();
            *count -= 1;
            pairs -= *count;
            left += 1;
        }
    }

    result
}