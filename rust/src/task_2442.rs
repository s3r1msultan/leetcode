/*You are given an array nums consisting of positive integers.

You have to take each integer in the array, reverse its digits, and add it to the end of the array. You should apply this operation to the original integers in nums.

Return the number of distinct integers in the final array.



Example 1:

Input: nums = [1,13,10,12,31]
Output: 6
Explanation: After including the reverse of each number, the resulting array is [1,13,10,12,31,1,31,1,21,13].
The reversed integers that were added to the end of the array are underlined. Note that for the integer 10, after reversing it, it becomes 01 which is just 1.
The number of distinct integers in this array is 6 (The numbers 1, 10, 12, 13, 21, and 31).

Example 2:

Input: nums = [2,2,2]
Output: 1
Explanation: After including the reverse of each number, the resulting array is [2,2,2,2,2,2].
The number of distinct integers in this array is 1 (The number 2).



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 106

*/

use std::collections::HashSet;

fn count_distinct_integers(nums: Vec<i32>) -> i32 {
	let mut set: HashSet<i32> = HashSet::with_capacity(nums.len()*2);
	for mut num in nums {
		set.insert(num);
		let mut reversed = 0;
		while num != 0 {
			reversed*=10;
			reversed+=num%10;
			num/=10;
		}
		set.insert(reversed);
	}
	set.len() as i32
}

#[cfg(test)]
#[test]
fn test_count_distinct_integers() {
	let nums = vec![1,13,10,12,31];
	let result = 6;
	assert_eq!(count_distinct_integers(nums), result);

	let nums = vec![2,2,2];
	let result = 1;
	assert_eq!(count_distinct_integers(nums), result);


}