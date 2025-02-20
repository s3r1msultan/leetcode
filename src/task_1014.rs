/*You are given an integer array values where values[i] represents the value of the ith sightseeing spot. Two sightseeing spots i and j have a distance j - i between them.

The score of a pair (i < j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them.

Return the maximum score of a pair of sightseeing spots.



Example 1:

Input: values = [8,1,5,2,6]
Output: 11
Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11

Example 2:

Input: values = [1,2]
Output: 2



Constraints:

2 <= values.length <= 5 * 104
1 <= values[i] <= 1000

*/

/*fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
	let mut start: (i32, i32) = (0, values[0]);
	let mut max_score = 0;
	for i in 1..values.len() {
		let score = start.0 + start.1 + values[i] - i as i32;
		if score > max_score {
			max_score = score;
		}
		if i as i32 + values[i] > start.0 + start.1 {
			start = (i as i32, values[i]);
		}
	}
	max_score
}*/

pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut max_score = 0;

    let mut start = (0, values[0]);

    for i in 1..values.len() {
        let score = start.0 + start.1 + values[i] - i as i32;
        max_score = max_score.max(score);
        if start.0 + start.1 < i as i32 + values[i] {
            start = (i as i32, values[i]);
        }
    }

    max_score
}





























