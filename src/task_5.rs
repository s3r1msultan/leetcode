/*Given a string s, return the longest palindromic substring in s.



Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.

Example 2:

Input: s = "cbbd"
Output: "bb"



Constraints:

1 <= s.length <= 1000
s consist of only digits and English letters.

*/


fn longest_palindrome(s: String) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let mut answer = String::new();
    for i in 0..s.len() {
        for j in i + answer.len()..s.len() {
            if s[i..=j].len() > answer.len() && s[i..=j].iter().rev().eq(s[i..=j].iter()) {
                answer = s[i..=j].iter().collect();
            }
        }
    }
    answer
}

#[cfg(test)]
#[test]
fn test_longest_palindrome() {
    let s = "babad".to_string();
    let result = "bab".to_string();
    assert_eq!(longest_palindrome(s), result);

    let s = "cbbd".to_string();
    let result = "bb".to_string();
    assert_eq!(longest_palindrome(s), result);
}