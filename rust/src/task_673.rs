/*Given an integer array nums, return the number of longest increasing subsequences.

Notice that the sequence has to be strictly increasing.



Example 1:

Input: nums = [1,3,5,4,7]
Output: 2
Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].

Example 2:

Input: nums = [2,2,2,2,2]
Output: 5
Explanation: The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.



Constraints:

1 <= nums.length <= 2000
-106 <= nums[i] <= 106
The answer is guaranteed to fit inside a 32-bit integer.

*/

fn find_number_of_lis(nums: Vec<i32>) -> i32 {
	let n = nums.len();
	let mut lengths = vec![1; n];
	let mut counts = vec![1; n];

	for i in 1..n {
		for j in 0..i {
			if nums[i] > nums[j] {
				if lengths[j] + 1 > lengths[i] {
					lengths[i] = lengths[j] + 1;
					counts[i] = 0;
				}
				if lengths[j] + 1 == lengths[i] {
					counts[i] += counts[j];
				}
			}
		}
	}
	println!("{lengths:?}");
	println!("{counts:?}");
	let max_length = *lengths.iter().max().unwrap();

	counts.iter().enumerate().fold(0, |acc, (i, &curr)| acc + if max_length == lengths[i] { curr } else { 0 })
}