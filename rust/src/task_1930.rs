/*Given a string s, return the number of unique palindromes of length three that are a subsequence of s.

Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.

A palindrome is a string that reads the same forwards and backwards.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

For example, "ace" is a subsequence of "abcde".



Example 1:

Input: s = "aabca"
Output: 3
Explanation: The 3 palindromic subsequences of length 3 are:
- "aba" (subsequence of "aabca")
- "aaa" (subsequence of "aabca")
- "aca" (subsequence of "aabca")

Example 2:

Input: s = "adc"
Output: 0
Explanation: There are no palindromic subsequences of length 3 in "adc".

Example 3:

Input: s = "bbcbaba"
Output: 4
Explanation: The 4 palindromic subsequences of length 3 are:
- "bbb" (subsequence of "bbcbaba")
- "bcb" (subsequence of "bbcbaba")
- "bab" (subsequence of "bbcbaba")
- "aba" (subsequence of "bbcbaba")



Constraints:

3 <= s.length <= 105
s consists of only lowercase English letters.

*/

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut first_occurrence = vec![-1; 26];
    let mut last_occurrence = vec![-1; 26];

    for i in 0..n {
        let letter = (bytes[i] - b'a') as usize;
        if first_occurrence[letter] == -1 {
            first_occurrence[letter] = i as i32;
        }
        last_occurrence[letter] = i as i32;
    }

    let mut count = 0;

    for i in 0..26 {
        if first_occurrence[i] == -1 {
            continue;
        }
        let mut set = std::collections::HashSet::new();
        for j in first_occurrence[i]..last_occurrence[i] {
            set.insert(bytes[j as usize]);
        }
        count += set.len() as i32;
    }


    count
}