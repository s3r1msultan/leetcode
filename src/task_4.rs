// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)).
//
//
//
// Example 1:
//
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
//
// Example 2:
//
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
//
//
// Constraints:
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106
//


pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
	let (mut nums1, mut nums2) = if nums1.len() <= nums2.len() {(nums1, nums2)} else {(nums2, nums1)};
	let (m, n) = (nums1.len(), nums2.len());

	let mut i_left = 0;
	let mut i_right = m;
	let half_len = (m+n+1)/2;
	let result = 0.0;

	while i_left <= i_right {
		let i_mid = (i_left + i_right) / 2;
		let j_left = half_len-1;

		if i_mid < m && nums1[i_mid] < nums2[j_left - 1] {
			i_left = i_mid + 1;
		} else if i_mid > 0 && nums1[i_mid - 1] < nums2[j_left] {
			i_right = i_mid - 1;
		} else {

		}
	}

	result

}

#[cfg(test)]

#[test]

fn test_find_median_sorted_arrays() {
	let nums1 = vec![1,3];
	let nums2 = vec![2];
	let result = 2.0;
	assert_eq!(find_median_sorted_arrays(nums1,nums2), result);

	let nums1 = vec![1,2];
	let nums2 = vec![3,4];
	let result = 2.5;
	assert_eq!(find_median_sorted_arrays(nums1,nums2), result);
}