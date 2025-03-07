/*Given a string s, find the longest palindromic subsequence's length in s.

A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.



Example 1:

Input: s = "bbbab"
Output: 4
Explanation: One possible longest palindromic subsequence is "bbbb".

Example 2:

Input: s = "cbbd"
Output: 2
Explanation: One possible longest palindromic subsequence is "bb".



Constraints:

1 <= s.length <= 1000
s consists only of lowercase English letters.

*/

pub fn longest_palindrome_subseq(s: String) -> i32 {
    fn lps(s: &str, i: i16, j: i16, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == j {
            return 1;
        }
        if i > j || i > s.len() as i16 || j < 0 {
            return 0;
        }
        
        let i = i as usize;
        let j = j as usize;
        
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        
        memo[i][j] = if s.as_bytes()[i] == s.as_bytes()[j] {
            2 + lps(s, i as i16 + 1, j as i16 - 1, memo)
        } else {
            lps(s, i as i16 + 1, j as i16, memo).max(lps(s, i as i16, j as i16 - 1, memo))
        };
        
        memo[i][j]
    }
    let n = s.len();
    let mut memo = vec![vec![-1; n]; n];
    lps(&s, 0, n as i16 - 1, &mut memo)
}