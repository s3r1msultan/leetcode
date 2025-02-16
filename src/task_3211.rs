/*You are given a positive integer n.

A binary string x is valid if all substrings of x of length 2 contain at least one "1".

Return all valid strings with length n, in any order.



Example 1:

Input: n = 3

Output: ["010","011","101","110","111"]

Explanation:

The valid strings of length 3 are: "010", "011", "101", "110", and "111".

Example 2:

Input: n = 1

Output: ["0","1"]

Explanation:

The valid strings of length 1 are: "0" and "1".



Constraints:

1 <= n <= 18

*/

pub fn valid_strings(n: i32) -> Vec<String> {
    fn backtrack(result: &mut Vec<String>, s: &mut Vec<char>, n: i32, ch: char) {
        if n == 0 {
            result.push(s.clone().into_iter().collect());
            return;
        }
        for digit in ['0', '1'] {
            if digit == '0' && ch == '0' {
                continue;
            }
            s.push(digit);
            backtrack(result, s, n - 1, digit);
            s.pop();
        }
    }
    let mut result = vec![];
    backtrack(&mut result, &mut vec![], n, '1');
    result
}