/*Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.



Example 1:

Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].

Example 2:

Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.



Constraints:

1 <= intervals.length <= 104
intervals[i].length == 2
0 <= starti <= endi <= 104

*/

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort();
    // println!("{intervals:?}");

    let mut result = vec![];
    let mut curr_start = intervals[0][0];
    let mut curr_end = intervals[0][1];
    for interval in intervals.iter().skip(1) {
        let next_start = interval[0];
        let next_end = interval[1];
        if curr_end < next_start {
            result.push(vec![curr_start, curr_end]);
            curr_start = next_start;
        }
        curr_end = curr_end.max(next_end);

    }
    result.push(vec![curr_start, curr_end]);

    result
}