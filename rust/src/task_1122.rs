/*Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.

Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2. Elements that do not appear in arr2 should be placed at the end of arr1 in ascending order.



Example 1:

Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
Output: [2,2,2,1,4,3,3,9,6,7,19]

Example 2:

Input: arr1 = [28,6,22,8,44,17], arr2 = [22,28,8,6]
Output: [22,28,8,6,17,44]



Constraints:

    1 <= arr1.length, arr2.length <= 1000
    0 <= arr1[i], arr2[i] <= 1000
    All the elements of arr2 are distinct.
    Each arr2[i] is in arr1.

*/

use std::collections::{BTreeMap};

fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
	let mut map = BTreeMap::new();
	for num in arr1 {
		*map.entry(num).or_insert(0)+=1;
	}
	let mut result = vec![];

	for &num in &arr2 {
		let count = *map.entry(num).or_insert(0);
		for _ in 0..count {
			result.push(num);
		}
		map.remove(&num);
	}

	for &num in map.keys() {
		if let Some(&count) = map.get(&num) {
			for _ in 0..count {
				result.push(num)
			}
		}
	}

	result
}

#[cfg(test)]
#[test]

fn test_relative_sort_array() {
	let arr1 = vec![2,3,1,3,2,4,6,7,9,2,19];
	let arr2 = vec![2,1,4,3,9,6];
	let result = vec![2,2,2,1,4,3,3,9,6,7,19];
	assert_eq!(relative_sort_array(arr1, arr2),result);

	let arr1 = vec![28,6,22,8,44,17];
	let arr2 = vec![22,28,8,6];
	let result = vec![22,28,8,6,17,44];
	assert_eq!(relative_sort_array(arr1, arr2),result);
}