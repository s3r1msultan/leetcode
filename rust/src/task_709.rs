/*Given a string s, return the string after replacing every uppercase letter with the same lowercase letter.



Example 1:

Input: s = "Hello"
Output: "hello"

Example 2:

Input: s = "here"
Output: "here"

Example 3:

Input: s = "LOVELY"
Output: "lovely"



Constraints:

1 <= s.length <= 100
s consists of printable ASCII characters.

*/

pub fn to_lower_case(s: String) -> String {
    let mut lowercase = String::new();
    for char in s.chars() {
        let char = char as u8;
        if char >= 65 && char <= 90 {
            lowercase.push((char + 32) as char);
            continue;
        }
        lowercase.push(char as char);
    }
    lowercase
}