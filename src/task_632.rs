/*You have k lists of sorted integers in non-decreasing order. Find the smallest range that includes at least one number from each of the k lists.

We define the range [a, b] is smaller than range [c, d] if b - a < d - c or a < c if b - a == d - c.



Example 1:

Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
Output: [20,24]
Explanation:
List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
List 2: [0, 9, 12, 20], 20 is in range [20,24].
List 3: [5, 18, 22, 30], 22 is in range [20,24].

Example 2:

Input: nums = [[1,2,3],[1,2,3],[1,2,3]]
Output: [1,1]



Constraints:

nums.length == k
1 <= k <= 3500
1 <= nums[i].length <= 50
-105 <= nums[i][j] <= 105
nums[i] is sorted in non-decreasing order.

*/pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
	let k = nums.len();
	let mut enumerated_nums = vec![];
	for i in 0..k {
		for &num in nums[i].iter() {
			enumerated_nums.push((num, i));
		}
	}

	enumerated_nums.sort();

	println!("{enumerated_nums:?}");
	let n = enumerated_nums.len();
	let mut frequencies = std::collections::HashMap::new();

	let mut left = 0;
	let mut count = 0;
	let mut range_start = 0;
	let mut range_end = i32::MAX;

	for right in 0..n {
		*frequencies.entry(enumerated_nums[right].1).or_insert(0) += 1;
		if frequencies.get(&enumerated_nums[right].1).unwrap() == &1 {
			count += 1;
		}

		while (count == k) {
			let curr_range = enumerated_nums[right].0 - enumerated_nums[left].0;
			if curr_range < range_end - range_start {
				range_start = enumerated_nums[left].0;
				range_end = enumerated_nums[right].0;
			}

			*frequencies.entry(enumerated_nums[left].1).or_insert(0) -= 1;

			if frequencies.get(&enumerated_nums[left].1).unwrap() == &0 {
				count -= 1;
			}

			left += 1;
		}
	}

	vec![range_start, range_end]
}