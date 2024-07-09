/*You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.

Return the number of consistent strings in the array words.



Example 1:

Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
Output: 2
Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.

Example 2:

Input: allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
Output: 7
Explanation: All strings are consistent.

Example 3:

Input: allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
Output: 4
Explanation: Strings "cc", "acd", "ac", and "d" are consistent.



Constraints:

1 <= words.length <= 104
1 <= allowed.length <= 26
1 <= words[i].length <= 10
The characters in allowed are distinct.
words[i] and allowed contain only lowercase English letters.

*/


fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for char in allowed.chars() {
        set.insert(char);
    }
    let n = set.len();
    let mut count = 0;

    for word in words {
        let mut current_count = 0;
        for char in word.chars() {
            if set.contains(&char) {
                current_count += 1;
            }
        }
        if current_count == n {
            count += 1;
        }
        current_count = 0;
    }

    count
}

#[cfg(test)]
#[test]
fn test_count_consistent_strings() {
    let allowed = "ab".to_string();
    let words = ["ad", "bd", "aaab", "baa", "badab"];
    let result = 2;
}