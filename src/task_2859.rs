// You are given a 0-indexed integer array nums and an integer k.
//
// Return an integer that denotes the sum of elements in nums whose corresponding indices have exactly k set bits in their binary representation.
//
// The set bits in an integer are the 1's present when it is written in binary.
//
// For example, the binary representation of 21 is 10101, which has 3 set bits.
//
//
//
// Example 1:
//
// Input: nums = [5,10,1,5,2], k = 1
// Output: 13
// Explanation: The binary representation of the indices are:
// 0 = 0002
// 1 = 0012
// 2 = 0102
// 3 = 0112
// 4 = 1002
// Indices 1, 2, and 4 have k = 1 set bits in their binary representation.
// Hence, the answer is nums[1] + nums[2] + nums[4] = 13.
//
// Example 2:
//
// Input: nums = [4,3,2,1], k = 2
// Output: 1
// Explanation: The binary representation of the indices are:
// 0 = 002
// 1 = 012
// 2 = 102
// 3 = 112
// Only index 3 has k = 2 set bits in its binary representation.
// Hence, the answer is nums[3] = 1.
//
//
//
// Constraints:
//
// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 105
// 0 <= k <= 10
//
// pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
// 	let mut result = 0;
// 	for i in 0..nums.len() {
// 		if i.count_ones() == k as u32 {
// 			result+=nums[i];
// 		}
// 	}
// 	result
// }

pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
	let k = k as u32;
	(0..nums.len()).filter(|&x| x.count_ones() == k).fold(0, |acc, curr| acc+nums[curr])
}


#[cfg(test)]
#[test]

fn test_sum_indeces_with_k_set_bits() {
	let nums = vec![5,10,1,5,2];
	let k = 1;
	let result = 13;
	assert_eq!(sum_indices_with_k_set_bits(nums, k), result);

	let nums = vec![4,3,2,1];
	let k = 2;
	let result = 1;
	assert_eq!(sum_indices_with_k_set_bits(nums, k), result);
}