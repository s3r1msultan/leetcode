/*You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part. For example, the string "ababcc" can be partitioned into ["abab", "cc"], but partitions such as ["aba", "bcc"] or ["ab", "ab", "cc"] are invalid.

Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.

Return a list of integers representing the size of these parts.



Example 1:

Input: s = "ababcbacadefegdehijhklij"
Output: [9,7,8]
Explanation:
The partition is "ababcbaca", "defegde", "hijhklij".
This is a partition so that each letter appears in at most one part.
A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.

Example 2:

Input: s = "eccbbbbdec"
Output: [10]



Constraints:

1 <= s.length <= 500
s consists of lowercase English letters.

*/

pub fn partition_labels(s: String) -> Vec<i32> {
    let n = s.len();
    let mut map = std::collections::HashMap::new();

    let s = s.as_bytes();

    for i in 0..n {
        *map.entry(s[i]).or_insert((i, i)).1 = i;
    }

    let mut chars = map.values().collect::<Vec<_>>();
    chars.sort_unstable();

    let mut result = vec![];
    let mut prev_start = chars[0].0;
    let mut prev_end = chars[0].1;
    for &(start, end) in chars {
        if start > prev_end {
            result.push((prev_end - prev_start + 1) as i32);
            prev_start = start;
        }

        prev_end = prev_end.max(end);
    }

    result
}