/*You are given a string s and an integer repeatLimit. Construct a new string repeatLimitedString using the characters of s such that no letter appears more than repeatLimit times in a row. You do not have to use all characters from s.

Return the lexicographically largest repeatLimitedString possible.

A string a is lexicographically larger than a string b if in the first position where a and b differ, string a has a letter that appears later in the alphabet than the corresponding letter in b. If the first min(a.length, b.length) characters do not differ, then the longer string is the lexicographically larger one.



Example 1:

Input: s = "cczazcc", repeatLimit = 3
Output: "zzcccac"
Explanation: We use all of the characters from s to construct the repeatLimitedString "zzcccac".
The letter 'a' appears at most 1 time in a row.
The letter 'c' appears at most 3 times in a row.
The letter 'z' appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return "zzcccac".
Note that the string "zzcccca" is lexicographically larger but the letter 'c' appears more than 3 times in a row, so it is not a valid repeatLimitedString.

Example 2:

Input: s = "aababab", repeatLimit = 2
Output: "bbabaa"
Explanation: We use only some of the characters from s to construct the repeatLimitedString "bbabaa".
The letter 'a' appears at most 2 times in a row.
The letter 'b' appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return "bbabaa".
Note that the string "bbabaaa" is lexicographically larger but the letter 'a' appears more than 2 times in a row, so it is not a valid repeatLimitedString.



Constraints:

1 <= repeatLimit <= s.length <= 105
s consists of lowercase English letters.

*/


pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    use std::collections::BinaryHeap;

    let mut result = String::new();

    let mut max_heap = BinaryHeap::new();
    let mut map = [0; 26];
    let bytes = s.as_bytes();

    for &byte in bytes {
        map[(byte - b'a') as usize] += 1;
    }

    for (ch, &count) in map.iter().enumerate() {
        if count != 0 {
            max_heap.push((ch, count));
        }
    }

    while let Some((ch, mut count)) = max_heap.pop() {
        let repeat_count = repeat_limit.min(count);
        for _ in 0..repeat_count {
            result.push((ch as u8 + b'a') as char);
        }
        count -= repeat_count;

        if count > 0 {
            if let Some((next_ch, mut next_count)) = max_heap.pop() {
                result.push((next_ch as u8 + b'a') as char);
                next_count -= 1;
                if next_count > 0 {
                    max_heap.push((next_ch, next_count));
                }
                max_heap.push((ch, count));
            }
        }
    }


    result
}
