/*Given an array of integers nums, sort the array in ascending order and return it.

You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.



Example 1:

Input: nums = [5,2,3,1]
Output: [1,2,3,5]
Explanation: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).

Example 2:

Input: nums = [5,1,1,2,0,0]
Output: [0,0,1,1,2,5]
Explanation: Note that the values of nums are not necessairly unique.



Constraints:

1 <= nums.length <= 5 * 104
-5 * 104 <= nums[i] <= 5 * 104

*/

fn sort_array(nums: Vec<i32>) -> Vec<i32> {
	fn merge_sort(array: &mut [i32]) {
		let len = array.len();
		if len > 1 {
			let mid = len / 2;
			merge_sort(&mut array[..mid]);
			merge_sort(&mut array[mid..]);
			merge(array, mid);
		}
	}

	fn merge(array: &mut [i32], mid: usize) {
		let mut left = array[..mid].to_vec();
		let mut right = array[mid..].to_vec();
		let (mut i, mut j, mut k) = (0, 0, 0);

		while i < left.len() && j < right.len() {
			if left[i] <= right[j] {
				array[k] = left[i];
				i += 1;
			} else {
				array[k] = right[j];
				j += 1;
			}
			k += 1;
		}

		while i < left.len() {
			array[k] = left[i];
			i += 1;
			k += 1;
		}

		while j < right.len() {
			array[k] = right[j];
			j += 1;
			k += 1;
		}
	}
	let mut nums = nums;
	let mut nums = nums.as_mut_slice();
	merge_sort(nums);
	nums.to_vec()
}