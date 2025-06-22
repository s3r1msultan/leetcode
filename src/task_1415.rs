/*A happy string is a string that:

consists only of letters of the set ['a', 'b', 'c'].
s[i] != s[i + 1] for all values of i from 1 to s.length - 1 (string is 1-indexed).

For example, strings "abc", "ac", "b" and "abcbabcbcb" are all happy strings and strings "aa", "baa" and "ababbc" are not happy strings.

Given two integers n and k, consider a list of all happy strings of length n sorted in lexicographical order.

Return the kth string of this list or return an empty string if there are less than k happy strings of length n.



Example 1:

Input: n = 1, k = 3
Output: "c"
Explanation: The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".

Example 2:

Input: n = 1, k = 4
Output: ""
Explanation: There are only 3 happy strings of length 1.

Example 3:

Input: n = 3, k = 9
Output: "cab"
Explanation: There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"



Constraints:

1 <= n <= 10
1 <= k <= 100

*/

pub fn get_happy_string(n: i32, k: i32) -> String {
    fn backtrack(curr_string: &mut String, prev_char: char, happy_letters: &[char; 3], n: usize, result: &mut Vec<String>) {
        if curr_string.len() == n {
            result.push(curr_string.clone());
            return;
        }

        for &char in happy_letters {
            if char == prev_char {
                continue;
            }
            curr_string.push(char);
            backtrack(curr_string, char, happy_letters, n, result);
            curr_string.pop();
        }
    }
    let n = n as usize;
    let happy_letters = ['a', 'b', 'c'];
    let mut result: Vec<String> = vec![];
    let mut curr_string = String::new();
    backtrack(&mut curr_string, '\0', &happy_letters, n, &mut result);
    let k = k as usize - 1;
    if let Some(s) = result.get(k) {
        s.clone()
    } else {
        "".to_string()
    }
}