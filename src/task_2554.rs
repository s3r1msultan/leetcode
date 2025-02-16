/*You are given an integer array banned and two integers n and maxSum. You are choosing some number of integers following the below rules:

The chosen integers have to be in the range [1, n].
Each integer can be chosen at most once.
The chosen integers should not be in the array banned.
The sum of the chosen integers should not exceed maxSum.

Return the maximum number of integers you can choose following the mentioned rules.



Example 1:

Input: banned = [1,6,5], n = 5, maxSum = 6
Output: 2
Explanation: You can choose the integers 2 and 4.
2 and 4 are from the range [1, 5], both did not appear in banned, and their sum is 6, which did not exceed maxSum.

Example 2:

Input: banned = [1,2,3,4,5,6,7], n = 8, maxSum = 1
Output: 0
Explanation: You cannot choose any integer while following the mentioned conditions.

Example 3:

Input: banned = [11], n = 7, maxSum = 50
Output: 7
Explanation: You can choose the integers 1, 2, 3, 4, 5, 6, and 7.
They are from the range [1, 7], all did not appear in banned, and their sum is 28, which did not exceed maxSum.



Constraints:

1 <= banned.length <= 104
1 <= banned[i], n <= 104
1 <= maxSum <= 109

*/
/*pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    use std::collections::HashSet;
    let banned = banned.into_iter().collect::<HashSet<i32>>();
    let mut k = 0;
    let mut curr_sum = 0;
    for i in 1..=n {
        if !banned.contains(&i) && curr_sum + i <= max_sum {
            curr_sum += i;
            k += 1;
        }
    }
    k
}*/

pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    fn is_banned(banned: &Vec<i32>, i: i32) -> bool {
        let mut start = 0;
        let mut end = banned.len();

        while start < end {
            let mid = start + (end - start) / 2;
            if banned[mid] == i {
                return true;
            } else if banned[mid] > i {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        false
    }

    let mut banned = banned;
    banned.sort_unstable();
    let mut k = 0;
    let mut curr_sum = 0;
    for i in 1..=n {
        if !is_banned(&banned, i) {
            if curr_sum + i <= max_sum {
                curr_sum += i;
                k += 1;
            } else {
                return k;
            }
        }
    }

    k
}