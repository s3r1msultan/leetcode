/*Given a string s, return true if s is a good string, or false otherwise.

A string s is good if all the characters that appear in s have the same number of occurrences (i.e., the same frequency).



Example 1:

Input: s = "abacbc"
Output: true
Explanation: The characters that appear in s are 'a', 'b', and 'c'. All characters occur 2 times in s.

Example 2:

Input: s = "aaabb"
Output: false
Explanation: The characters that appear in s are 'a' and 'b'.
'a' occurs 3 times while 'b' occurs 2 times, which is not the same number of times.



Constraints:

1 <= s.length <= 1000
s consists of lowercase English letters.*/

pub fn are_occurrences_equal(s: String) -> bool {
    let mut letters = [0; 26];
    let bytes = s.as_bytes();
    let mut max = 0;
    for &byte in bytes {
        let i = (byte - b'a') as usize;
        letters[i] += 1;
        max = max.max(letters[i]);
    }
    for i in 0..26 {
        if letters[i] != max {
            return false;
        }
    }
    true
}