/*Given an integer array nums of unique elements, return all possible subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.



Example 1:

Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

Example 2:

Input: nums = [0]
Output: [[],[0]]



Constraints:

    1 <= nums.length <= 10
    -10 <= nums[i] <= 10
    All the numbers of nums are unique.

*/

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
	fn backtrack( start: usize, nums: &Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
		result.push(current.clone());
		for i in start..nums.len() {
			current.push(nums[i]);
			backtrack(i+1, nums, current, result);
			current.pop();
		}
	}
	let mut result = vec![];
	let mut current = vec![];
	backtrack(0, &nums, &mut current, &mut result);
	result
}

#[cfg(test)]

#[test]
fn test_subsets() {
	let nums = vec![1,2,3];
	let result = vec![vec![],vec![1],vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]];
	assert_eq!(subsets(nums), result);

	let nums = vec![0];
	let result = vec![vec![],vec![0]];
	assert_eq!(subsets(nums), result);
}