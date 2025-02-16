/*Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
frequency
of at least one of the chosen numbers is different.

The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.



Example 1:

Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
Explanation:
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.

Example 2:

Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]

Example 3:

Input: candidates = [2], target = 1
Output: []



Constraints:

1 <= candidates.length <= 30
2 <= candidates[i] <= 40
All elements of candidates are distinct.
1 <= target <= 40

*/
/*
fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    fn backtrack(candidates: &Vec<i32>, target: i32, current_sum: i32, current_candidates: &mut Vec<i32>, result: &mut HashSet<Vec<i32>>) {
        if current_sum > target {
            return;
        }
        if current_sum == target {
            let mut current_candidates = current_candidates.clone();
            current_candidates.sort();
            result.insert(current_candidates.clone());
            return;
        }
        for i in 0..candidates.len() {
            current_candidates.push(candidates[i]);
            backtrack(candidates, target, current_sum + candidates[i], current_candidates, result);
            current_candidates.pop();
        }
    }

    let mut result = HashSet::new();
    backtrack(&candidates, target, 0, &mut Vec::new(), &mut result);
    result.into_iter().collect()
}*/

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    fn backtrack(nums: &Vec<i32>, target: i32, start: usize, result: &mut Vec<Vec<i32>>, current_nums: &mut Vec<i32>) {
        if target == 0 {
            result.push(current_nums.clone());
            return
        }

        for i in start..nums.len() {
            let num = nums[i];
            if num > target {
                return;
            }
            current_nums.push(num);
            backtrack(nums, target - num, i, result, current_nums);
            current_nums.pop();
        }
    }
    let mut result = vec![];
    backtrack(&candidates, target, 0, &mut result, &mut Vec::new());
    result
}