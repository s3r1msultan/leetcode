/*Given a string s, partition the string into one or more substrings such that the characters in each substring are unique. That is, no letter appears in a single substring more than once.

Return the minimum number of substrings in such a partition.

Note that each character should belong to exactly one substring in a partition.



Example 1:

Input: s = "abacaba"
Output: 4
Explanation:
Two possible partitions are ("a","ba","cab","a") and ("ab","a","ca","ba").
It can be shown that 4 is the minimum number of substrings needed.

Example 2:

Input: s = "ssssss"
Output: 6
Explanation:
The only valid partition is ("s","s","s","s","s","s").



Constraints:

1 <= s.length <= 105
s consists of only English lowercase letters.

*/

// solution using Hash Set
// fn partition_string(s: String) -> i32 {
//     use std::collections::HashSet;
//     let mut set = HashSet::new();
//     let mut count = 1;
//     for char in s.chars() {
//         if set.contains(&char) {
//             set.clear();
//             count += 1;
//         }
//         set.insert(char);
//     }
//     count
// }

// solution using array of chars
fn partition_string(s: String) -> i32 {
    let mut array = [false; 26];
    let mut count = 1;
    for char in s.chars() {
        if array[(char as u8 - 'a' as u8) as usize] {
            count += 1;
            array.fill(false);
        }

        array[(char as u8 - 'a' as u8) as usize] = true;
    }
    count
}


#[cfg(test)]
#[test]
fn test_partition_string() {
    let s = "abacada".to_string();
    let result = 4;
    assert_eq!(partition_string(s), result);
    let s = "ssssss".to_string();
    let result = 6;
    assert_eq!(partition_string(s), result);
    let s = "hdklqkcssgxlvehva".to_string();
    let result = 4;
    assert_eq!(partition_string(s), result);
}