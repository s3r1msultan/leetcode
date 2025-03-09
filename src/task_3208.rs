/*There is a circle of red and blue tiles. You are given an array of integers colors and an integer k. The color of tile i is represented by colors[i]:

colors[i] == 0 means that tile i is red.
colors[i] == 1 means that tile i is blue.

An alternating group is every k contiguous tiles in the circle with alternating colors (each tile in the group except the first and last one has a different color from its left and right tiles).

Return the number of alternating groups.

Note that since colors represents a circle, the first and the last tiles are considered to be next to each other.



Example 1:

Input: colors = [0,1,0,1,0], k = 3

Output: 3

Explanation:

Alternating groups:

Example 2:

Input: colors = [0,1,0,0,1,0,1], k = 6

Output: 2

Explanation:

Alternating groups:

Example 3:

Input: colors = [1,1,0,1], k = 4

Output: 0

Explanation:



Constraints:

3 <= colors.length <= 105
0 <= colors[i] <= 1
3 <= k <= colors.length

*/

pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = colors.len();
    let mut count = 0;
    let mut left = 0;
    for right in 1..n + k - 1 {
        if colors[right % n] == colors[(right - 1) % n] {
            left = right;
            continue;
        } else if right - left + 1 == k {
            left += 1;
            count += 1;
        }
    }
    count
}