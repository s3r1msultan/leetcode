/*Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.

A shift on s consists of moving the leftmost character of s to the rightmost position.

For example, if s = "abcde", then it will be "bcdea" after one shift.



Example 1:

Input: s = "abcde", goal = "cdeab"
Output: true

Example 2:

Input: s = "abcde", goal = "abced"
Output: false



Constraints:

1 <= s.length, goal.length <= 100
s and goal consist of lowercase English letters.

*/


pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let n = s.len();
    for i in 0..n {
        if &s[n - i..] == &goal[0..i] && &s[0..n - i] == &goal[i..] {
            return true;
        }
    }


    false
}