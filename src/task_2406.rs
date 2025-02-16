/*You are given a 2D integer array intervals where intervals[i] = [lefti, righti] represents the inclusive interval [lefti, righti].

You have to divide the intervals into one or more groups such that each interval is in exactly one group, and no two intervals that are in the same group intersect each other.

Return the minimum number of groups you need to make.

Two intervals intersect if there is at least one common number between them. For example, the intervals [1, 5] and [5, 8] intersect.



Example 1:

Input: intervals = [[5,10],[6,8],[1,5],[2,3],[1,10]]
Output: 3
Explanation: We can divide the intervals into the following groups:
- Group 1: [1, 5], [6, 8].
- Group 2: [2, 3], [5, 10].
- Group 3: [1, 10].
It can be proven that it is not possible to divide the intervals into fewer than 3 groups.

Example 2:

Input: intervals = [[1,3],[5,6],[8,10],[11,13]]
Output: 1
Explanation: None of the intervals overlap, so we can put all of them in one group.



Constraints:

1 <= intervals.length <= 105
intervals[i].length == 2
1 <= lefti <= righti <= 106

*/

fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals_with_ends = vec![];
    for interval in intervals {
        intervals_with_ends.push((interval[0], 1));
        intervals_with_ends.push((interval[1] + 1, -1));
    }

    intervals_with_ends.sort();
    println!("{intervals_with_ends:?}");
    let n = intervals_with_ends.len();
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n+1 {
        prefix_sum[i+1] = prefix_sum[i] + intervals_with_ends[i][1];
    }
    println!("{prefix_sum:?}");

    *prefix_sum.iter().max().unwrap()
}