/*You are given two strings s and t. In one step, you can append any character to either s or t.

Return the minimum number of steps to make s and t anagrams of each other.

An anagram of a string is a string that contains the same characters with a different (or the same) ordering.



Example 1:

Input: s = "leetcode", t = "coats"
Output: 7
Explanation:
- In 2 steps, we can append the letters in "as" onto s = "leetcode", forming s = "leetcodeas".
- In 5 steps, we can append the letters in "leede" onto t = "coats", forming t = "coatsleede".
"leetcodeas" and "coatsleede" are now anagrams of each other.
We used a total of 2 + 5 = 7 steps.
It can be shown that there is no way to make them anagrams of each other with less than 7 steps.

Example 2:

Input: s = "night", t = "thing"
Output: 0
Explanation: The given strings are already anagrams of each other. Thus, we do not need any further steps.



Constraints:

1 <= s.length, t.length <= 2 * 105
s and t consist of lowercase English letters.

*/

pub fn min_steps(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut map_1 = [0i32; 26];
    let mut map_2 = [0i32; 26];

    for &byte in s {
        let i = (byte - b'a') as usize;
        map_1[i] += 1;
    }

    for &byte in t {
        let i = (byte - b'a') as usize;
        map_2[i] += 1;
    }

    let mut steps: i32 = 0;

    for i in 0..26 {
        steps += (map_1[i] - map_2[i]).abs();
    }

    steps
}