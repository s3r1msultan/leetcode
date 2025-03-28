/*Given a string s consisting only of characters a, b and c.

Return the number of substrings containing at least one occurrence of all these characters a, b and c.



Example 1:

Input: s = "abcabc"
Output: 10
Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).

Example 2:

Input: s = "aaacb"
Output: 3
Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".

Example 3:

Input: s = "abc"
Output: 1



Constraints:

3 <= s.length <= 5 x 10^4
s only consists of a, b or c characters.

*/

pub fn number_of_substrings(s: String) -> i32 {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut map = std::collections::HashMap::new();

    let mut count = 0;
    let mut left = 0;
    for right in 0..n {
        *map.entry(bytes[right]).or_insert(0) += 1;

        while map.len() == 3 {
            if Some(&1) == map.get(&bytes[left]) {  
                map.remove(&bytes[left]);
            } else {
                *map.entry(bytes[left]).or_insert(0) += 1;
            }
            count += n - right;
            left += 1;
        }
    }
    count as i32
}