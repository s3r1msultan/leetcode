/*Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

For example, "ace" is a subsequence of "abcde".

A common subsequence of two strings is a subsequence that is common to both strings.



Example 1:

Input: text1 = "abcde", text2 = "ace"
Output: 3
Explanation: The longest common subsequence is "ace" and its length is 3.

Example 2:

Input: text1 = "abc", text2 = "abc"
Output: 3
Explanation: The longest common subsequence is "abc" and its length is 3.

Example 3:

Input: text1 = "abc", text2 = "def"
Output: 0
Explanation: There is no such common subsequence, so the result is 0.



Constraints:

1 <= text1.length, text2.length <= 1000
text1 and text2 consist of only lowercase English characters.

*/

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let n = text1.len();
    let m = text2.len();
    let mut memo = vec![vec![-1; m]; n];
    let chars_1 = text1.chars().collect::<Vec<_>>();
    let chars_2 = text2.chars().collect::<Vec<_>>();
    fn lcs(chars_1: &Vec<char>, chars_2: &Vec<char>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        let n = chars_1.len();
        let m = chars_2.len();
        if n == 0 || m == 0 {
            return 0;
        }

        if memo[n-1][m-1] != -1 {
            return memo[n-1][m-1];
        }

        memo[n-1][m-1] = if chars_1[n-1] == chars_2[m-1] {
            1 + lcs(&chars_1[..n-1].to_vec(), &chars_2[..m-1].to_vec(), n-1, m-1, memo)
        } else {
            lcs(&chars_1[..n-1].to_vec(), chars_2, n-1, m-1, memo).max(lcs(chars_1, &chars_2[..m-1].to_vec(), n-1, m-1, memo))
        };
        memo[n-1][m-1]
    }

    lcs(&chars_1, &chars_2, n-1, m-1, &mut memo)
}