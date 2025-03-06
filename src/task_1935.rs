/*There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken, return the number of words in text you can fully type using this keyboard.



Example 1:

Input: text = "hello world", brokenLetters = "ad"
Output: 1
Explanation: We cannot type "world" because the 'd' key is broken.

Example 2:

Input: text = "leet code", brokenLetters = "lt"
Output: 1
Explanation: We cannot type "leet" because the 'l' and 't' keys are broken.

Example 3:

Input: text = "leet code", brokenLetters = "e"
Output: 0
Explanation: We cannot type either word because the 'e' key is broken.



Constraints:

1 <= text.length <= 104
0 <= brokenLetters.length <= 26
text consists of words separated by a single space without any leading or trailing spaces.
Each word only consists of lowercase English letters.
brokenLetters consists of distinct lowercase English letters.

*/

pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut letters = [false; 26];
    let bytes = broken_letters.as_bytes();
    bytes.iter().for_each(|&b| {
       letters[(b-b'a') as usize] = true;
    });

    let mut count = 0;
    let words = text.split_whitespace().collect::<Vec<_>>();
    for word in words {
        let bytes = word.as_bytes();
        let mut has_broken_letters = false;
        for &byte in bytes {
            let i = (byte - b'a') as usize;
            if letters[i] {
                has_broken_letters = true;
                break;
            }
        }
        if !has_broken_letters {
            count += 1;
        }
    }
    count
}