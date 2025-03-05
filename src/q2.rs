/*You are given an array original of length n and a 2D array bounds of length n x 2, where bounds[i] = [ui, vi].

You need to find the number of possible arrays copy of length n such that:

(copy[i] - copy[i - 1]) == (original[i] - original[i - 1]) for 1 <= i <= n - 1.
ui <= copy[i] <= vi for 0 <= i <= n - 1.

Return the number of such arrays.



Example 1:

Input: original = [1,2,3,4], bounds = [[1,2],[2,3],[3,4],[4,5]]

Output: 2

Explanation:

The possible arrays are:

[1, 2, 3, 4]
[2, 3, 4, 5]

Example 2:

Input: original = [1,2,3,4], bounds = [[1,10],[2,9],[3,8],[4,7]]

Output: 4

Explanation:

The possible arrays are:

[1, 2, 3, 4]
[2, 3, 4, 5]
[3, 4, 5, 6]
[4, 5, 6, 7]

Example 3:

Input: original = [1,2,1,2], bounds = [[1,1],[2,3],[3,3],[2,3]]

Output: 0

Explanation:

No array is possible.



Constraints:

2 <= n == original.length <= 105
1 <= original[i] <= 109
bounds.length == n
bounds[i].length == 2
1 <= bounds[i][0] <= bounds[i][1] <= 109©leetcode*/

    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let n = original.len();
        let mut diffs = vec![];
        for i in 1..n {
            diffs.push(original[i] - original[i-1]);
        }

        let mut low = bounds[n-1][0];
        let mut high = bounds[n-1][1];

        for i in (1..n).rev() {
            let curr_low = bounds[i-1][0];
            let curr_high = bounds[i-1][1];

            let new_low = curr_low.max(low - diffs[i-1]);
            let new_high = curr_high.min(high - diffs[i-1]);

            if new_low > new_high {
                return 0;
            }

            low = new_low;
            high = new_high;
        }

        high - low + 1
    }
