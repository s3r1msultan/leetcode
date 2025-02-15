/*You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.

Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.



Example 1:

Input: s1 = "bank", s2 = "kanb"
Output: true
Explanation: For example, swap the first character with the last character of s2 to make "bank".

Example 2:

Input: s1 = "attack", s2 = "defend"
Output: false
Explanation: It is impossible to make them equal with one string swap.

Example 3:

Input: s1 = "kelb", s2 = "kelb"
Output: true
Explanation: The two strings are already equal, so no string swap operation is required.



Constraints:

1 <= s1.length, s2.length <= 100
s1.length == s2.length
s1 and s2 consist of only lowercase English letters.

*/

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let n = s1.len();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let mut first_letters = [0; 26];
    let mut second_letters = [0; 26];

    for i in 0..n {
        let letter1 = (s1[i] - b'a') as usize;
        let letter2 = (s2[i] - b'a') as usize;
        first_letters[letter1] += 1;
        second_letters[letter2] += 1;
    }

    for i in 0..n {
        if first_letters[i] != second_letters[i] {
            return false;
        }
    }

    let mut count = 0;
    for i in 0..n {
        if s1[i] != s2[i] {
            count += 1;
        }
    }

    count <= 2
}