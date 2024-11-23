/*Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.



Example 1:

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

Example 2:

Input: n = 1
Output: ["()"]



Constraints:

1 <= n <= 8

*/

 /*   pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(result: &mut Vec<String>, brackets: &mut String, l: i32, r: i32, n: i32) {
            if l == n && r == n{
                result.push(brackets.clone());
                return;
            }
            if l < n {
                brackets.push('(');
                backtrack(result, brackets, l+1, r, n);
                brackets.pop();
            }

            if r < l {
                brackets.push('(');
                backtrack(result, brackets, l, r+1, n);
                brackets.pop();
            }

        }
        let mut result = vec![];
        let mut brackets = vec![];
        backtrack(&mut result, &mut brackets, 0, 0, n);
        result
    }
*/