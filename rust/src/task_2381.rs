/*
You are given a string s of lowercase English letters and a 2D integer array shifts where shifts[i] = [starti, endi, directioni]. For every i, shift the characters in s from the index starti to the index endi (inclusive) forward if directioni = 1, or shift the characters backward if directioni = 0.

Shifting a character forward means replacing it with the next letter in the alphabet (wrapping around so that 'z' becomes 'a'). Similarly, shifting a character backward means replacing it with the previous letter in the alphabet (wrapping around so that 'a' becomes 'z').

Return the final string after all such shifts to s are applied.



Example 1:

Input: s = "abc", shifts = [[0,1,0],[1,2,1],[0,2,1]]
Output: "ace"
Explanation: Firstly, shift the characters from index 0 to index 1 backward. Now s = "zac".
Secondly, shift the characters from index 1 to index 2 forward. Now s = "zbd".
Finally, shift the characters from index 0 to index 2 forward. Now s = "ace".

Example 2:

Input: s = "dztz", shifts = [[0,0,0],[1,1,1]]
Output: "catz"
Explanation: Firstly, shift the characters from index 0 to index 0 backward. Now s = "cztz".
Finally, shift the characters from index 1 to index 1 forward. Now s = "catz".



Constraints:

1 <= s.length, shifts.length <= 5 * 104
shifts[i].length == 3
0 <= starti <= endi < s.length
0 <= directioni <= 1
s consists of lowercase English letters.
*/

pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut number_of_shifts = vec![0; n + 1];
    for shift in shifts {
        let start = shift[0] as usize;
        let end = shift[1] as usize;
        let direction = if shift[2] == 0 {
            -1
        } else {
            1
        };

        number_of_shifts[start] += direction;
        number_of_shifts[end + 1] -= direction;
    }
    let mut cumulative_shift = 0;
    let bytes = s.as_bytes();
    let mut result = vec![];
    for i in 0..n {
        cumulative_shift += number_of_shifts[i] % 26;
        let new_letter = ((bytes[i] - b'a') as i32 + cumulative_shift + 26) as u8 % 26 + b'a';
        result.push(new_letter as char);
    }
    result.into_iter().collect()
}
