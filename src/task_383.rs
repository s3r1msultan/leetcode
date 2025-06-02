/*Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

Each letter in magazine can only be used once in ransomNote.



Example 1:

Input: ransomNote = "a", magazine = "b"
Output: false

Example 2:

Input: ransomNote = "aa", magazine = "ab"
Output: false

Example 3:

Input: ransomNote = "aa", magazine = "aab"
Output: true



Constraints:

1 <= ransomNote.length, magazine.length <= 105
ransomNote and magazine consist of lowercase English letters.

*/

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let ransom_note = ransom_note.as_bytes();
    let magazine = magazine.as_bytes();
    let mut map_1 = std::collections::HashMap::new();
    let mut map_2 = std::collections::HashMap::new();

    for &byte in ransom_note {
        *map_1.entry(byte).or_insert(0) += 1;
    }

    for &byte in magazine {
        *map_2.entry(byte).or_insert(0) += 1;
    }

    for (val, count_1) in map_1 {
        if let Some(&count_2) = map_2.get(&val) {
            if count_1 < count_2 {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}