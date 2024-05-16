// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
//
// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:
//
// The 1st place athlete's rank is "Gold Medal".
// The 2nd place athlete's rank is "Silver Medal".
// The 3rd place athlete's rank is "Bronze Medal".
// For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
//
// Return an array answer of size n where answer[i] is the rank of the ith athlete.
//
//
//
// Example 1:
//
// Input: score = [5,4,3,2,1]
// Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
// Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].
//
// Example 2:
//
// Input: score = [10,3,8,9,4]
// Output: ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
// Explanation: The placements are [1st, 5th, 3rd, 2nd, 4th].
//
//
//
// Constraints:
//
// n == score.length
// 1 <= n <= 104
// 0 <= score[i] <= 106
// All the values in score are unique.
//
pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
	let mut indexed_scores: Vec<(usize, i32)> = score.iter().enumerate().map(|(i, &s)| (i,s)).collect();
	indexed_scores.sort_by(|a, b| b.1.cmp(&a.1));
	let mut result = vec![String::new(); score.len()];
	for (rank, (index, _)) in indexed_scores.iter().enumerate() {
		result[*index] = match rank {
			0 => "Gold Medal".to_string(),
			1 => "Silver Medal".to_string(),
			2 => "Bronze Medal".to_string(),
			_ => (rank+1).to_string(),
		};
	}
	result
}

#[cfg(test)]

#[test]

fn test_find_relative_ranks() {
	let score = vec![5,4,3,2,1];
	let result: Vec<String> = ["Gold Medal","Silver Medal","Bronze Medal","4","5"].iter().map(|&s| s.to_string()).collect();
	assert_eq!(find_relative_ranks(score), result);

	let score = vec![10,3,8,9,4];
	let result: Vec<String> =["Gold Medal","5","Bronze Medal","Silver Medal","4"].iter().map(|&s| s.to_string()).collect();
	assert_eq!(find_relative_ranks(score), result);
}