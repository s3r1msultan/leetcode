/*Given a string s, reverse only all the vowels in the string and return it.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.



Example 1:

Input: s = "hello"
Output: "holle"

Example 2:

Input: s = "leetcode"
Output: "leotcede"



Constraints:

1 <= s.length <= 3 * 105
s consist of printable ASCII characters.

*/

/*fn reverse_vowels(s: String) -> String {
    let vowels = HashSet::from(['a', 'e', 'u', 'o', 'i']);
    let s: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = s.len() - 1;
    let mut left_part = "".to_string();
    let mut right_part = "".to_string();
    while left <= right && left + 1 < s.len() && right - 1 >= 0 {
        if !vowels.contains(&s[left]) {
            left_part.push(s[left]);
            left += 1;
            continue;
        }
        if !vowels.contains(&s[right]) {
            right_part.push(s[right]);
            right -= 1;
            continue;
        }
        if right != left {
            left_part.push(s[right]);
            right_part.push(s[left]);
            left += 1;
            right -= 1;
        } else {
            left_part.push(s[left]);
            left += 1;
        }
    }
    left_part += &*right_part.chars().rev().collect::<String>();
    left_part
}
*/

use std::collections::HashSet;

fn reverse_vowels(s: String) -> String {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'e']);
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut tuple = vec![];
    let n = chars.len();
    for i in 0..n {
        if vowels.contains(&chars[i]) {
            tuple.push((i, chars[i]));
        }
    }
    for i in 0..tuple.len() / 2 {
        let (m, l) = tuple[i];
        let (n, r) = tuple[tuple.len() - i - 1];
        chars[m] = r;
        chars[n] = l;
    }
    chars.iter().collect()
}

#[cfg(test)]
#[test]
fn test_reverse_vowels() {
    let s = "hello".to_string();
    let result = "holle".to_string();
    assert_eq!(reverse_vowels(s), result);
    let s = "leetcode".to_string();
    let result = "leotcede".to_string();
    assert_eq!(reverse_vowels(s), result);
}