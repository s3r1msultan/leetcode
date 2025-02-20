/*You are given a string s consisting of lowercase English letters. Your task is to find the maximum difference between the frequency of two characters in the string such that:

One of the characters has an even frequency in the string.
The other character has an odd frequency in the string.

Return the maximum difference, calculated as the frequency of the character with an odd frequency minus the frequency of the character with an even frequency.



Example 1:

Input: s = "aaaaabbc"

Output: 3

Explanation:

The character 'a' has an odd frequency of 5, and 'b' has an even frequency of 2.
The maximum difference is 5 - 2 = 3.

Example 2:

Input: s = "abcabcab"

Output: 1

Explanation:

The character 'a' has an odd frequency of 3, and 'c' has an even frequency of 2.
The maximum difference is 3 - 2 = 1.



Constraints:

3 <= s.length <= 100
s consists only of lowercase English letters.
s contains at least one character with an odd frequency and one with an even frequency.

*/

pub fn max_difference(s: String) -> i32 {
    let mut letters = [0;26];
    let bytes = s.as_bytes();

    for &byte in bytes {
        let i = (byte - b'a') as usize;
        letters[i] += 1;
    }

    let mut min_even = i32::MAX;
    let mut max_odd = i32::MIN;

    for i in 0..26 {
        if letters[i] == 0 {
            continue;
        }
        if letters[i] % 2 == 0 {
            min_even = min_even.min(letters[i]);
        } else {
            max_odd = max_odd.max(letters[i]);
        }
    }

    max_odd - min_even
}