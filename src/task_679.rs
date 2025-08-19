/* You are given an integer array cards of length 4. You have four cards, each containing a number in the range [1, 9]. You should arrange the numbers on these cards in a mathematical expression using the operators ['+', '-', '*', '/'] and the parentheses '(' and ')' to get the value 24.

You are restricted with the following rules:

    The division operator '/' represents real division, not integer division.
        For example, 4 / (1 - 2 / 3) = 4 / (1 / 3) = 12.
    Every operation done is between two numbers. In particular, we cannot use '-' as a unary operator.
        For example, if cards = [1, 1, 1, 1], the expression "-1 - 1 - 1 - 1" is not allowed.
    You cannot concatenate numbers together
        For example, if cards = [1, 2, 1, 2], the expression "12 + 12" is not valid.

Return true if you can get such expression that evaluates to 24, and false otherwise.



Example 1:

Input: cards = [4,1,8,7]
Output: true
Explanation: (8-4) * (7-1) = 24

Example 2:

Input: cards = [1,2,1,2]
Output: false



Constraints:

    cards.length == 4
    1 <= cards[i] <= 9

*/

pub fn judge_point24(cards: Vec<i32>) -> bool {
    fn backtrack(
        curr_numerator: i32,
        mut curr_denominator: i32,
        i: usize,
        nums: &Vec<i32>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if i == nums.len() {
            return curr_numerator / curr_denominator == 24
                && curr_numerator % curr_denominator == 0;
        }

        if curr_numerator == 0 {
            curr_denominator = 1;
        }

        for i in 0..4 {
            if visited[i] {
                continue;
            }

            visited[i] = true;

            // Addition
            // first case: we include the sum
            backtrack(curr_numerator, curr_denominator, i, nums, visited);

            // second case: we exclude the sum
            backtrack(curr_numerator, curr_denominator, i, nums, visited);

            // Substraction
            // include the substraction
            // exclude the substraction

            visited[i] = false;
        }

        false
    }

    let mut visited = vec![false; 4];
    backtrack(0, 1, 0, &cards, &mut visited)
}
