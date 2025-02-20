/*You are given a string s consisting of the characters 'a', 'b', and 'c' and a non-negative integer k. Each minute, you may take either the leftmost character of s, or the rightmost character of s.

Return the minimum number of minutes needed for you to take at least k of each character, or return -1 if it is not possible to take k of each character.



Example 1:

Input: s = "aabaaaacaabc", k = 2
Output: 8
Explanation:
Take three characters from the left of s. You now have two 'a' characters, and one 'b' character.
Take five characters from the right of s. You now have four 'a' characters, two 'b' characters, and two 'c' characters.
A total of 3 + 5 = 8 minutes is needed.
It can be proven that 8 is the minimum number of minutes needed.

Example 2:

Input: s = "a", k = 1
Output: -1
Explanation: It is not possible to take one 'b' or 'c' so return -1.



Constraints:

1 <= s.length <= 105
s consists of only the letters 'a', 'b', and 'c'.
0 <= k <= s.length

*/
pub fn take_characters(s: String, k: i32) -> i32 {
    let mut counts = vec![0; 3];

    let chars = s.as_bytes();
    for &char in chars {
        counts[(char - b'a') as usize] += 1;
    }

    if counts[0] < k || counts[1] < k || counts[2] < k {
        return -1;
    }

    let mut count = i32::MAX;
    let mut left = 0;
    for right in 0..chars.len() {
        counts[(chars[right] - b'a') as usize] -= 1;

        while counts[(chars[right] - b'a') as usize] < k {
            counts[(chars[left] - b'a') as usize] += 1;
            left += 1;
        }

        count = count.min((chars.len() - right + left - 1) as i32);
    }
    count
}