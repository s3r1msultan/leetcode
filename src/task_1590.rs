/*Given an array of positive integers nums, remove the smallest subarray (possibly empty) such that the sum of the remaining elements is divisible by p. It is not allowed to remove the whole array.

Return the length of the smallest subarray that you need to remove, or -1 if it's impossible.

A subarray is defined as a contiguous block of elements in the array.*/


fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
	let mut min = i32::MAX;
	let mut map = std::collections::HashMap::new();
	map.insert(0, -1);
	let p = p as i64;
	let mut target = nums.iter().map(|&val| val as i64).sum::<i64>() % p;
	if target == 0 {
		return 0;
	}
	let mut prefix = 0;
	for (i, &val) in nums.iter().enumerate() {
		prefix = (prefix + val as i64) % p;
		let need = (prefix - target + p) % p;
		if let Some(&j) = map.get(&need) {
			min = min.min(i as i32 - j);
		}
		map.insert(prefix, i as i32);
	}
	if min == nums.len() as i32 {
		-1
	} else {
		min
	}
}