/*Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.



Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

Example 2:

Input: digits = ""
Output: []

Example 3:

Input: digits = "2"
Output: ["a","b","c"]



Constraints:

0 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].

*/

pub fn letter_combinations(digits: String) -> Vec<String> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert('2', vec!['a', 'b', 'c']);
    map.insert('3', vec!['d', 'e', 'f']);
    map.insert('4', vec!['g', 'h', 'i']);
    map.insert('5', vec!['j', 'k', 'l']);
    map.insert('6', vec!['m', 'n', 'o']);
    map.insert('7', vec!['p', 'q', 'r', 's']);
    map.insert('8', vec!['t', 'u', 'v']);
    map.insert('9', vec!['w', 'x', 'y', 'z']);

    fn backtrack(digits: &Vec<char>, map: &HashMap<char, Vec<char>>, start: usize, current_combination: &mut Vec<char>, result: &mut Vec<String>) {
        if current_combination.len() == digits.len() {
            result.push(current_combination.iter().collect());
            return;
        }
        let chars = map.get(&digits[start]).unwrap();
        for &char in chars {
            current_combination.push(char);
            backtrack(digits, map, start + 1, current_combination, result);
            current_combination.pop();
        }
    }
    let digits = digits.chars().collect::<Vec<char>>();
    let mut result = vec![];
    let mut current_combination = vec![];
    if !digits.is_empty() {
        backtrack(&digits, &map, 0, &mut current_combination, &mut result);
    }
    result
}