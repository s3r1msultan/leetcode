/*You are given a string s and a pattern string p, where p contains exactly one '*' character.

The '*' in p can be replaced with any sequence of zero or more characters.

Return true if p can be made a substring of s, and false otherwise.

A substring is a contiguous non-empty sequence of characters within a string.



Example 1:

Input: s = "leetcode", p = "ee*e"

Output: true

Explanation:

By replacing the '*' with "tcod", the substring "eetcode" matches the pattern.

Example 2:

Input: s = "car", p = "c*v"

Output: false

Explanation:

There is no substring matching the pattern.

Example 3:

Input: s = "luck", p = "u*"

Output: true

Explanation:

The substrings "u", "uc", and "uck" match the pattern.



Constraints:

1 <= s.length <= 50
1 <= p.length <= 50
s contains only lowercase English letters.
p contains only lowercase English letters and exactly one '*'©leetcode*/

pub fn has_match(s: String, p: String) -> bool {
    if s.as_bytes()[0] == b'*' {
        let suffix = &s[1..];
        for i in 0..s.len() {
            if s[i..].starts_with(suffix) {
                return true;
            }
        }
    } else if s.as_bytes().last().unwrap() == &b'*' {
        let prefix = &s[..s.len() - 1];
        for i in 0..s.len() {
            if s[i..].starts_with(prefix) {
                return true;
            }
        }
    } else {
        let parts: Vec<&str> = s.split("*").collect();
        let (prefix, suffix) = (parts.get(0).unwrap_or(&""), parts.get(1).unwrap_or(&""));

        for i in 0..s.len() {
            if s[i..].starts_with(prefix) {
                let remaining = &s[i + prefix.len()..];
                if remaining.ends_with(suffix) {
                    return true;
                }
            }
        }
    }


    false
}