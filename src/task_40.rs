/*Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.

Each number in candidates may only be used once in the combination.

Note: The solution set must not contain duplicate combinations.



Example 1:

Input: candidates = [10,1,2,7,6,1,5], target = 8
Output:
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]

Example 2:

Input: candidates = [2,5,2,1,2], target = 5
Output:
[
[1,2,2],
[5]
]



Constraints:

1 <= candidates.length <= 100
1 <= candidates[i] <= 50
1 <= target <= 30

*/

fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
	let mut result = vec![];
	let mut path = vec![];
	let mut candidates = candidates;
	candidates.sort_unstable();

	fn backtrack(candidates: &Vec<i32>, path: &mut Vec<i32>, start: usize, target: i32, result: &mut Vec<Vec<i32>>) {
		if target == 0 {
			result.push(path.clone());
			return;
		}

		for i in start..candidates.len() {
			if i > start && candidates[i] == candidates[i - 1] {
				continue;
			}

			if target < candidates[i] {
				break;
			}

			path.push(candidates[i]);
			backtrack(candidates, path, i + 1, target - candidates[i], result);
			path.pop();
		}
	}

	backtrack(&candidates, &mut path, 0, target, &mut result);
	result
}