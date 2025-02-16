/*You are given two integer arrays of equal length target and arr. In one step, you can select any non-empty subarray of arr and reverse it. You are allowed to make any number of steps.

Return true if you can make arr equal to target or false otherwise.



Example 1:

Input: target = [1,2,3,4], arr = [2,4,1,3]
Output: true
Explanation: You can follow the next steps to convert arr to target:
1- Reverse subarray [2,4,1], arr becomes [1,4,2,3]
2- Reverse subarray [4,2], arr becomes [1,2,4,3]
3- Reverse subarray [4,3], arr becomes [1,2,3,4]
There are multiple ways to convert arr to target, this is not the only way to do so.

Example 2:

Input: target = [7], arr = [7]
Output: true
Explanation: arr is equal to target without any reverses.

Example 3:

Input: target = [3,7,9], arr = [3,7,11]
Output: false
Explanation: arr does not have value 9 and it can never be converted to target.



Constraints:

target.length == arr.length
1 <= target.length <= 1000
1 <= target[i] <= 1000
1 <= arr[i] <= 1000

*/

// fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
// 	use std::collections::HashMap;
// 	let mut target_map: HashMap<i32, i32> = HashMap::new();
// 	let mut arr_map: HashMap<i32, i32> = HashMap::new();
//
// 	for (&target_num, &arr_num) in target.iter().zip(arr.iter()) {
// 		*target_map.entry(target_num).or_insert(0) += 1;
// 		*arr_map.entry(arr_num).or_insert(0) += 1;
// 	}
//
// 	for (key, value) in target_map {
// 		if let Some(&arr_value) = arr_map.get(key) {
// 			if arr_value != value {
// 				return false;
// 			}
// 		} else {
// 			return false;
// 		}
// 	}
// 	true
// }

fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
	target.sort_unstable();
	arr.sort_unstable();
	target == arr
}