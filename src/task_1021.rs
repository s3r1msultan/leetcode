/*A valid parentheses string is either empty "", "(" + A + ")", or A + B, where A and B are valid parentheses strings, and + represents string concatenation.

For example, "", "()", "(())()", and "(()(()))" are all valid parentheses strings.

A valid parentheses string s is primitive if it is nonempty, and there does not exist a way to split it into s = A + B, with A and B nonempty valid parentheses strings.

Given a valid parentheses string s, consider its primitive decomposition: s = P1 + P2 + ... + Pk, where Pi are primitive valid parentheses strings.

Return s after removing the outermost parentheses of every primitive string in the primitive decomposition of s.



Example 1:

Input: s = "(()())(())"
Output: "()()()"
Explanation:
The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
After removing outer parentheses of each part, this is "()()" + "()" = "()()()".

Example 2:

Input: s = "(()())(())(()(()))"
Output: "()()()()(())"
Explanation:
The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".

Example 3:

Input: s = "()()"
Output: ""
Explanation:
The input string is "()()", with primitive decomposition "()" + "()".
After removing outer parentheses of each part, this is "" + "" = "".



Constraints:

1 <= s.length <= 105
s[i] is either '(' or ')'.
s is a valid parentheses string.

*/

pub fn remove_outer_parentheses(s: String) -> String {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut depths = vec![0; n];
    let mut curr_depth = 1;
    for i in 0..n {
        if bytes[i] == b'(' {
            depths[i] = curr_depth;
            curr_depth += 1;
        } else {
            curr_depth -= 1;
            depths[i] = curr_depth;
        }
    }

    depths = depths.into_iter().map(|d| d - 1).collect();

    let mut result = "".to_string();
    curr_depth = 0;
    for i in 0..n {
        if curr_depth == depths[i] {
            result.push(')');
            curr_depth -= 1;
        } else {
            result.push('(');
            curr_depth += 1;
        }
    }

    result
}