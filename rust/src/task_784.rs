/*Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.

Return a list of all possible strings we could create. Return the output in any order.



Example 1:

Input: s = "a1b2"
Output: ["a1b2","a1B2","A1b2","A1B2"]

Example 2:

Input: s = "3z4"
Output: ["3z4","3Z4"]



Constraints:

1 <= s.length <= 12
s consists of lowercase English letters, uppercase English letters, and digits.

*/
//
// pub fn letter_case_permutation(s: String) -> Vec<String> {
//     fn backtrack(result: &mut Vec<String>, current_string: &mut String, i: usize, s: &Vec<char>) {
//         if i == s.len() {
//             result.push(current_string.clone());
//             return;
//         }
//
//         if s[i].is_ascii_digit() {
//             current_string.push(s[i]);
//             backtrack(result, current_string, i + 1, s);
//             current_string.pop();
//         } else {
//             current_string.push(s[i].to_ascii_lowercase());
//             backtrack(result, current_string, i + 1, s);
//             current_string.pop();
//
//             current_string.push(s[i].to_ascii_uppercase());
//             backtrack(result, current_string, i + 1, s);
//             current_string.pop();
//         }
//     }
//     let mut result = Vec::new();
//     let mut current_string = "".to_string();
//     let chars = s.chars().collect::<Vec<char>>();
//     backtrack(&mut result, &mut current_string, 0, &chars);
//     result.into_iter().collect()
// }

pub fn letter_case_permutation(s: String) -> Vec<String> {
    fn backtrack(chars: &mut Vec<char>, i: usize, result: &mut Vec<String>) {
        if i == chars.len() {
            result.push(chars.clone().into_iter().collect());
        } else if chars[i].is_ascii_digit() {
            backtrack(chars, i + 1, result);
        } else {
            chars[i] = chars[i].to_ascii_uppercase();
            backtrack(chars, i + 1, result);
            chars[i] = chars[i].to_ascii_lowercase();
            backtrack(chars, i + 1, result);
        }
    }
    let mut result = vec![];
    backtrack(&mut s.chars().collect(), 0, &mut result);
    result
}