/*Given a string s and an integer k, return true if you can use all the characters in s to construct k palindrome strings or false otherwise.



Example 1:

Input: s = "annabelle", k = 2
Output: true
Explanation: You can construct two palindromes using all characters in s.
Some possible constructions "anna" + "elble", "anbna" + "elle", "anellena" + "b"

Example 2:

Input: s = "leetcode", k = 3
Output: false
Explanation: It is impossible to construct 3 palindromes using all the characters of s.

Example 3:

Input: s = "true", k = 4
Output: true
Explanation: The only possible solution is to put each character in a separate string.



Constraints:

1 <= s.length <= 105
s consists of lowercase English letters.
1 <= k <= 105

*/

pub fn can_construct(s: String, k: i32) -> bool {
    if s.len() < k as usize {
        return false;
    }

    let mut frequencies = [0; 26];

    for &byte in s.as_bytes() {
        let i = (byte - b'a') as usize;
        frequencies[i] += 1;
    }

    let mut odd_frequencies = 0;
    for i in 0..26 {
        if frequencies[i] % 2 == 1 {
            odd_frequencies += 1;
        }
    }

    odd_frequencies < k
}