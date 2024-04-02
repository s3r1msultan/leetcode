// Given two strings s and t, determine if they are isomorphic.
//
// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
//
// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
//
//
//
// Example 1:
//
// Input: s = "egg", t = "add"
// Output: true
//
// Example 2:
//
// Input: s = "foo", t = "bar"
// Output: false
//
// Example 3:
//
// Input: s = "paper", t = "title"
// Output: true
//
//
//
// Constraints:
//
// 1 <= s.length <= 5 * 104
// t.length == s.length
// s and t consist of any valid ascii character.
//

use std::collections::HashMap;
// First solution


// pub fn is_isomorphic(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }
//
//     let s = s.split("").collect::<Vec<&str>>();
//     let t = t.split("").collect::<Vec<&str>>();
//
//     let mut map1 = HashMap::<&str, &str>::new();
//     let mut map2 = HashMap::<&str, &str>::new();
//     for i in 0..s.len() {
//         if map1.contains_key(s[i]) && map2.contains_key(t[i]) {
//             if map1.get(s[i]).unwrap() != &t[i] || map2.get(t[i]).unwrap() != &s[i] {
//                 return false;
//             }
//         } else if map1.contains_key(s[i]) || map2.contains_key(t[i]) {
//             return false;
//         } else {
//             map1.insert(s[i], t[i]);
//             map2.insert(t[i], s[i]);
//         }
//     }
//
//     true
// }


// Second solution

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for (char1, char2) in s.chars().zip(t.chars()) {
        if let Some(&mapped_char) = map1.get(&char1) {
            if mapped_char != char2 {
                return false
            }
        } else {
            map1.insert(char1, char2);
        }

        if let Some(&mapped_char) = map2.get(&char2) {
            if mapped_char != char1 {
                return false;
            }
        } else {
            map2.insert(char2, char1);
        }
    }

    true
}

#[cfg(test)]

#[test]

fn test_is_isomorphic() {
    let s = "egg".to_string();
    let t = "add".to_string();
    assert!(is_isomorphic(s,t));

    let s = "paper".to_string();
    let t = "title".to_string();
    assert!(is_isomorphic(s,t));

    let s = "foo".to_string();
    let t = "bar".to_string();
    assert!(!is_isomorphic(s, t));
}