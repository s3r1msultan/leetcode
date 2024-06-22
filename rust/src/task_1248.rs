/*Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.

Return the number of nice sub-arrays.



Example 1:

Input: nums = [1,1,2,1,1], k = 3
Output: 2
Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].

Example 2:

Input: nums = [2,4,6], k = 1
Output: 0
Explanation: There are no odd numbers in the array.

Example 3:

Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
Output: 16



Constraints:

1 <= nums.length <= 50000
1 <= nums[i] <= 10^5
1 <= k <= nums.length

*/

use std::collections::HashMap;

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
	let mut prefix_sum_counts = HashMap::new();
	prefix_sum_counts.insert(0, 1);
	let mut prefix_sum = 0;
	let mut current_prefix_sum = 0;

	for num in nums {
		if num % 2 == 1 {
			current_prefix_sum += 1;
		}

		if let Some(&occurrences) = prefix_sum_counts.get(&(current_prefix_sum - k)) {
			prefix_sum += occurrences;
		}

		*prefix_sum_counts.entry(current_prefix_sum).or_insert(0) += 1;
	}


	prefix_sum
}


// WRONG IDEA
// fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
// 	let mut count = 0;
// 	let mut prefix_sum_vector: Vec<i32> = vec![0];
//
// 	for i in 0..nums.len() {
// 		if nums[i] % 2 == 1 {
// 			prefix_sum_vector.push(prefix_sum_vector.last().unwrap() + 1);
// 		}
// 	}
//
// 	for i in 0..prefix_sum_vector.len() {
// 		for j in i..prefix_sum_vector.len() {
// 			if prefix_sum_vector[i].abs_diff(prefix_sum_vector[j]) as i32 == k {
// 				count += 1;
// 			} else if prefix_sum_vector[i].abs_diff(prefix_sum_vector[j]) as i32 > k {
// 				break;
// 			}
// 		}
// 	}
//
// 	println!("{:?}", prefix_sum_vector);
//
// 	count
// }


#[cfg(test)]
#[test]
fn test_number_of_subarrays() {
	let nums = vec![1, 1, 2, 1, 1];
	let k = 3;
	let result = 2;
	assert_eq!(number_of_subarrays(nums, k), result);

	let nums = vec![2, 4, 6];
	let k = 1;
	let result = 0;
	assert_eq!(number_of_subarrays(nums, k), result);

	let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
	let k = 2;
	let result = 16;
	assert_eq!(number_of_subarrays(nums, k), result);
}