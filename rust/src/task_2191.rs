/*You are given a 0-indexed integer array mapping which represents the mapping rule of a shuffled decimal system. mapping[i] = j means digit i should be mapped to digit j in this system.

The mapped value of an integer is the new integer obtained by replacing each occurrence of digit i in the integer with mapping[i] for all 0 <= i <= 9.

You are also given another integer array nums. Return the array nums sorted in non-decreasing order based on the mapped values of its elements.

Notes:

Elements with the same mapped values should appear in the same relative order as in the input.
The elements of nums should only be sorted based on their mapped values and not be replaced by them.



Example 1:

Input: mapping = [8,9,4,0,2,1,3,5,7,6], nums = [991,338,38]
Output: [338,38,991]
Explanation:
Map the number 991 as follows:
1. mapping[9] = 6, so all occurrences of the digit 9 will become 6.
2. mapping[1] = 9, so all occurrences of the digit 1 will become 9.
Therefore, the mapped value of 991 is 669.
338 maps to 007, or 7 after removing the leading zeros.
38 maps to 07, which is also 7 after removing leading zeros.
Since 338 and 38 share the same mapped value, they should remain in the same relative order, so 338 comes before 38.
Thus, the sorted array is [338,38,991].

Example 2:

Input: mapping = [0,1,2,3,4,5,6,7,8,9], nums = [789,456,123]
Output: [123,456,789]
Explanation: 789 maps to 789, 456 maps to 456, and 123 maps to 123. Thus, the sorted array is [123,456,789].



Constraints:

mapping.length == 10
0 <= mapping[i] <= 9
All the values of mapping[i] are unique.
1 <= nums.length <= 3 * 104
0 <= nums[i] < 109

*/

// fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
// 	use std::collections::HashMap;
// 	fn shuffle_digits(num: i32, shuffled_digits: &HashMap<i32, i32>) -> i32 {
// 		let mut new_num = 0;
// 		let binding = num.to_string();
// 		let mut num = binding.chars();
// 		for digit in num {
// 			new_num *= 10;
// 			new_num += shuffled_digits.get(&((digit as u8 - b'0' as u8) as i32)).unwrap();
// 		}
// 		new_num
// 	}
// 	let mut shuffled_digits = HashMap::new();
// 	for (initial, &shuffled_digit) in mapping.iter().enumerate() {
// 		shuffled_digits.insert(initial as i32, shuffled_digit);
// 	}
//
//
// 	let mut shuffled_nums = vec![];
// 	for num in nums {
// 		shuffled_nums.push((num, shuffle_digits(num, &shuffled_digits)));
// 	}
//
// 	shuffled_nums.sort_by(|a, b| a.1.cmp(&b.1));
// 	shuffled_nums.iter().map(|x| x.0).collect()
// }

fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
	let shuffle_num = |n: i32| {
		if n == 0 {
			return mapping[0];
		}
		let mut n = n;
		let mut result = 0;
		let mut degree = 1;
		while n > 0 {
			result += degree * mapping[(n % 10) as usize];
			n /= 10;
		}
		result
	};
	let mut nums = nums;
	nums.sort_by_key(|&num| shuffle_num(num));
	nums
}

#[cfg(test)]
#[test]
fn test_sort_jumbled() {
	let mapping = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
	let nums = vec![991, 338, 38];
	let result = vec![338, 38, 991];
	assert_eq!(sort_jumbled(mapping, nums), result);

	let mapping = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let nums = vec![789, 456, 123];
	let result = vec![123, 456, 789];
	assert_eq!(sort_jumbled(mapping, nums), result);

	let mapping = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
	let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let result = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
	assert_eq!(sort_jumbled(mapping, nums), result);
}