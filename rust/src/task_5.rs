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
    if s.len() < 0 {
        return "".to_string();
    }

    let chars = s.chars().collect();
    let mut start = 0;
    let mut end = 0;

    for i in s.len() {
        let len1 = expand_around_center(&chars, i, i);
        let len2 = expand_around_center(&chars, i, i + 1);
        let len = len1.max(len2);

        if len > end - start { start = i -  }
    }
    fn expand_around_center(chars: &[char], left: usize, right: usize) -> usize {
        let (mut l, mut r) = (left as isize, right as isize);
        let n = chars.len();
        while left as isize >= 0 && right < n && chars[l as usize] == chars[r as usize] {
            r -= 1;
            l += 1;
        }

        (r - l - 1) as usize
    }
    "".to_string()
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