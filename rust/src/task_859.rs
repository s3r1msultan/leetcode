/*Given two strings s and goal, return true if you can swap two letters in s so the result is equal to goal, otherwise, return false.

Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at s[i] and s[j].

For example, swapping at indices 0 and 2 in "abcd" results in "cbad".



Example 1:

Input: s = "ab", goal = "ba"
Output: true
Explanation: You can swap s[0] = 'a' and s[1] = 'b' to get "ba", which is equal to goal.

Example 2:

Input: s = "ab", goal = "ab"
Output: false
Explanation: The only letters you can swap are s[0] = 'a' and s[1] = 'b', which results in "ba" != goal.

Example 3:

Input: s = "aa", goal = "aa"
Output: true
Explanation: You can swap s[0] = 'a' and s[1] = 'a' to get "aa", which is equal to goal.



Constraints:

1 <= s.length, goal.length <= 2 * 104
s and goal consist of lowercase letters.

*/

pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let n = s.len();
    let s = s.as_bytes();
    let goal = goal.as_bytes();
    let mut letters_1 = [0; 26];
    let mut letters_2 = [0; 26];

    for i in 0..n {
        let letter1 = (s[i] - b'a') as usize;
        let letter2 = (goal[i] - b'a') as usize;
        letters_1[letter1] += 1;
        letters_2[letter2] += 1;
    }

    let mut count_1 = 0;

    for i in 0..26 {
        if letters_1[i] != letters_2[i] {
            return false;
        }
        if letters_1[i] > 1 {
            count_1 += 1;
        }
    }

    let mut count_2 = 0;

    for i in 0..n {
        if s[i] != goal[i] {
            count_2 += 1;
        }
    }

    if count_2 == 2 {
        return true;
    }
    if count_2 > 2 {
        return false;
    }
    count_1 > 0
}