/*Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.



Example 1:

Input: nums = [1,1,2]
Output:
[[1,1,2],
[1,2,1],
[2,1,1]]

Example 2:

Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]



Constraints:

1 <= nums.length <= 8
-10 <= nums[i] <= 10

*/

// pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     use std::collections::HashSet;
//     let mut result = HashSet::new();
//
//     fn backtrack(nums: &Vec<i32>, curr_permutation: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, set: &mut HashSet<usize>) {
//         if curr_permutation.len() == nums.len() {
//             result.push(curr_permutation.clone());
//             return;
//         }
//         for i in 0..nums.len() {
//             if !set.contains(&i) {
//                 set.insert(i);
//                 curr_permutation.push(nums[i]);
//                 backtrack(&nums, curr_permutation, result, set);
//                 curr_permutation.pop();
//                 set.remove(&i);
//             }
//         }
//     }
//
//     backtrack(&nums, &mut vec![], &mut result, &mut HashSet::new());
//
//     result.iter().collect()
// }

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut result = vec![];
    let mut permutation = vec![];
    let mut used = vec![false; nums.len()];

    fn backtrack(nums: &Vec<i32>, used: &mut Vec<bool>, permutation: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if permutation.len() == nums.len() {
            result.push(permutation.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] || i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            permutation.push(nums[i]);
            backtrack(nums, used, permutation, result);
            permutation.pop();
            used[i] = false;
        }
    }
    backtrack(&nums, &mut used, &mut permutation, &mut result);
    result
}