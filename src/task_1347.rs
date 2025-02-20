/*You are given two strings of the same length s and t. In one step you can choose any character of t and replace it with another character.

Return the minimum number of steps to make t an anagram of s.

An Anagram of a string is a string that contains the same characters with a different (or the same) ordering.



Example 1:

Input: s = "bab", t = "aba"
Output: 1
Explanation: Replace the first 'a' in t with b, t = "bba" which is anagram of s.

Example 2:

Input: s = "leetcode", t = "practice"
Output: 5
Explanation: Replace 'p', 'r', 'a', 'i' and 'c' from t with proper characters to make t anagram of s.

Example 3:

Input: s = "anagram", t = "mangaar"
Output: 0
Explanation: "anagram" and "mangaar" are anagrams.



Constraints:

1 <= s.length <= 5 * 104
s.length == t.length
s and t consist of lowercase English letters only.

*/

pub fn min_steps(s: String, t: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut map1 = vec![0; 26];
    let mut map2 = vec![0; 26];

    for i in 0..n {
        let a = (s[i] - b'a') as usize;
        let b = (t[i] - b'a') as usize;

        map1[a] += 1;
        map2[b] += 1;
    }

    let mut steps = 0;

    for i in 0..26 {
        steps += (map1[i] - map2[i]).max(0);
    }

    steps
}