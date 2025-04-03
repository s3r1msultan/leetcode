/*Given two strings s and t, return true if t is an

of s, and false otherwise.



Example 1:

Input: s = "anagram", t = "nagaram"

Output: true

Example 2:

Input: s = "rat", t = "car"

Output: false



Constraints:

1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.



Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?*/

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let n = s.len();
    let s = s.as_bytes();
    let t =t.as_bytes();
    let mut letters = [0;26];
    for i in 0..n {
        letters[(s[i] - b'a') as usize] += 1;
        letters[(t[i] - b'a') as usize] -= 1;
    }
    letters.into_iter().all(|x| x == 0)
}