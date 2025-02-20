/*A boolean expression is an expression that evaluates to either true or false. It can be in one of the following shapes:

't' that evaluates to true.
'f' that evaluates to false.
'!(subExpr)' that evaluates to the logical NOT of the inner expression subExpr.
'&(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical AND of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
'|(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical OR of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.

Given a string expression that represents a boolean expression, return the evaluation of that expression.

It is guaranteed that the given expression is valid and follows the given rules.



Example 1:

Input: expression = "&(|(f))"
Output: false
Explanation:
First, evaluate |(f) --> f. The expression is now "&(f)".
Then, evaluate &(f) --> f. The expression is now "f".
Finally, return false.

Example 2:

Input: expression = "|(f,f,f,t)"
Output: true
Explanation: The evaluation of (false OR false OR false OR true) is true.

Example 3:

Input: expression = "!(&(f,t))"
Output: true
Explanation:
First, evaluate &(f,t) --> (false AND true) --> false --> f. The expression is now "!(f)".
Then, evaluate !(f) --> NOT false --> true. We return true.



Constraints:

1 <= expression.length <= 2 * 104
expression[i] is one following characters: '(', ')', '&', '|', '!', 't', 'f', and ','.

*/
struct Solution {

}
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let chars = expression.chars().collect::<Vec<_>>();
        let mut start = 0;
        Self::helper(&chars, &mut start)
    }

    fn helper(chars: &Vec<char>, start: &mut usize) -> bool {
        match chars[*start] {
            '!' => {
                *start += 2; // skipping "!("
                let result = !Self::helper(chars, start);
                *start += 1; // skipping ")"
                result
            },
            '&' => {
                // here should be initial "true" for prev_expression parameter
                // it should return false if one of the listed booleans is false

                *start += 2; // skipping "&("
                while chars[*start] != ')' {
                    if chars[*start] == ',' {
                        *start += 1;
                    } else {
                        if !Self::helper(chars, start) {
                            while chars[*start] != ')' {
                                *start += 1; // skipping to the end boolean value
                            }
                            return false;
                        }
                        *start += 1;
                    }
                }
                *start += 1; // skipping ")"
                true
            },
            '|' => {
                // here should be initial "false" for prev_expression parameter
                // it should return true if one of the listed booleans is true

                *start += 2; // skipping "|("
                while chars[*start] != ')' {
                    if chars[*start] == ',' {
                        *start += 1;
                    } else {
                        if Self::helper(chars, start) {
                            while chars[*start] != ')' {
                                *start += 1; // skipping to the end boolean value
                            }
                            return true;
                        }
                        *start += 1;
                    }
                }
                *start += 1; // skipping ")"
                false
            },
            'f' => {
                false
            },
            't' => {
                true
            },
            _ => {
                println!("{start}");
                let left_chars = &chars[*start..].to_vec().into_iter().collect::<String>();
                println!("{left_chars:?}");
                unreachable!();
            }
        }
    }
}