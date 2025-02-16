// You are given a string s consisting of lowercase letters and an integer k. We call a string t ideal if the following conditions are satisfied:
//
// t is a subsequence of the string s.
// The absolute difference in the alphabet order of every two adjacent letters in t is less than or equal to k.
//
// Return the length of the longest ideal string.
//
// A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
//
// Note that the alphabet order is not cyclic. For example, the absolute difference in the alphabet order of 'a' and 'z' is 25, not 1.
//
//
//
// Example 1:
//
// Input: s = "acfgbd", k = 2
// Output: 4
// Explanation: The longest ideal string is "acbd". The length of this string is 4, so 4 is returned.
// Note that "acfgbd" is not ideal because 'c' and 'f' have a difference of 3 in alphabet order.
//
// Example 2:
//
// Input: s = "abcd", k = 3
// Output: 4
// Explanation: The longest ideal string is "abcd". The length of this string is 4, so 4 is returned.
//
//
//
// Constraints:
//
// 1 <= s.length <= 105
// 0 <= k <= 25
// s consists of lowercase English letters.
//


pub fn longest_ideal_string(s: String, k: i32) -> i32 {
    let mut dp = vec![0;26];
    let k = k as usize;
    for ch in s.chars() {
        let index = (ch as u8 - b'a') as usize;
        let mut max_length = 0;
        let start = if index > k { index - k } else { 0 };
        let end = usize::min(25, index + k);
        for j in start..=end {
            max_length = max_length.max(dp[j]);
        }
        dp[index] = max_length + 1;
    }

    dp.into_iter().max().unwrap_or(0)
}

#[cfg(test)]

#[test]

fn test_longest_ideal_string() {
    let s = "acfgbd".to_string();
    let k = 2;
    assert_eq!(longest_ideal_string(s, k), 4);

    let s = "eduktdb".to_string();
    let k = 15;
    assert_eq!(longest_ideal_string(s, k), 5);
}