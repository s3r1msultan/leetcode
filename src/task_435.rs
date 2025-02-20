/*Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.

Note that intervals which only touch at a point are non-overlapping. For example, [1, 2] and [2, 3] are non-overlapping.



Example 1:

Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
Output: 1
Explanation: [1,3] can be removed and the rest of the intervals are non-overlapping.

Example 2:

Input: intervals = [[1,2],[1,2],[1,2]]
Output: 2
Explanation: You need to remove two [1,2] to make the rest of the intervals non-overlapping.

Example 3:

Input: intervals = [[1,2],[2,3]]
Output: 0
Explanation: You don't need to remove any of the intervals since they're already non-overlapping.



Constraints:

1 <= intervals.length <= 105
intervals[i].length == 2
-5 * 104 <= starti < endi <= 5 * 104

*/

pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let n = intervals.len();
    let mut intervals = intervals;
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    let mut count = 0;
    let mut prev_end = intervals[0][1];
    for i in 0..n {
        let curr_start = intervals[i][0];
        let curr_end = intervals[i][1];
        if curr_start < prev_end {
            prev_end = curr_end.min(prev_end);
            count += 1;
        } else {
            prev_end = curr_end;
        }
    }
    count
}