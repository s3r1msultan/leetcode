/*Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.



Example 1:

Input: s = "Let's take LeetCode contest"
Output: "s'teL ekat edoCteeL tsetnoc"

Example 2:

Input: s = "Mr Ding"
Output: "rM gniD"



Constraints:

1 <= s.length <= 5 * 104
s contains printable ASCII characters.
s does not contain any leading or trailing spaces.
There is at least one word in s.
All the words in s are separated by a single space.

*/

pub fn reverse_words(s: String) -> String {
    let mut result = vec![];

    for string in s.split_whitespace() {
        let bytes = string.as_bytes();
        let n = bytes.len();
        let mut reversed_word = "".to_string();

        for i in (0..n).rev() {
            reversed_word.push(bytes[i] as char);
        }

        result.push(reversed_word);
    }

    result.join(" ")
}