/*You are given a 0-indexed string pattern of length n consisting of the characters 'I' meaning increasing and 'D' meaning decreasing.

A 0-indexed string num of length n + 1 is created using the following conditions:

num consists of the digits '1' to '9', where each digit is used at most once.
If pattern[i] == 'I', then num[i] < num[i + 1].
If pattern[i] == 'D', then num[i] > num[i + 1].

Return the lexicographically smallest possible string num that meets the conditions.



Example 1:

Input: pattern = "IIIDIDDD"
Output: "123549876"
Explanation:
At indices 0, 1, 2, and 4 we must have that num[i] < num[i+1].
At indices 3, 5, 6, and 7 we must have that num[i] > num[i+1].
Some possible values of num are "245639871", "135749862", and "123849765".
It can be proven that "123549876" is the smallest possible num that meets the conditions.
Note that "123414321" is not possible because the digit '1' is used more than once.

Example 2:

Input: pattern = "DDD"
Output: "4321"
Explanation:
Some possible values of num are "9876", "7321", and "8742".
It can be proven that "4321" is the smallest possible num that meets the conditions.



Constraints:

1 <= pattern.length <= 8
pattern consists of only the letters 'I' and 'D'.

*/


pub fn smallest_number(pattern: String) -> String {
    fn backtrack(curr_num:&mut Vec<char>, pattern: &Vec<char>, used: &mut [bool;9], result: &mut String,) {
        if curr_num.len() > pattern.len() {
            for i in 0..pattern.len() {
                if pattern[i] == 'I' {
                    if curr_num[i] > curr_num[i+1] {
                        return;
                    }
                } else {
                    if curr_num[i] < curr_num[i+1] {
                        return;
                    }
                }
            }
            *result = curr_num.clone().into_iter().collect();
            return;
        }
        for i in (0..9).rev() {
            if used[i] {
                continue;
            }
            used[i] = true;
            curr_num.push((i as u8 + b'1') as char);
            backtrack(curr_num, pattern, used, result);
            curr_num.pop();
            used[i] = false;
        }
    }
    let pattern = pattern.chars().collect::<Vec<_>>();
    let mut used = [false; 9];
    let mut curr_num = vec![];
    let mut result = "".to_string();
    backtrack(&mut curr_num, &pattern, &mut used, &mut result);
    result
}