/*You are given a string s and two integers x and y. You can perform two types of operations any number of times.

Remove substring "ab" and gain x points.
For example, when removing "ab" from "cabxbae" it becomes "cxbae".
Remove substring "ba" and gain y points.
For example, when removing "ba" from "cabxbae" it becomes "cabxe".

Return the maximum points you can gain after applying the above operations on s.



Example 1:

Input: s = "cdbcbbaaabab", x = 4, y = 5
Output: 19
Explanation:
- Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
- Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
- Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
- Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
Total score = 5 + 4 + 5 + 5 = 19.

Example 2:

Input: s = "aabbaaxybbaabb", x = 5, y = 4
Output: 20



Constraints:

1 <= s.length <= 105
1 <= x, y <= 104
s consists of lowercase English letters.

*/


fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut max = 0;
    let mut stack = vec![];
    if x > y {
        for char in s.chars() {
            if !stack.is_empty() && stack.last() == Some(&'a') && char == 'b' {
                stack.pop();
                max += x;
            } else {
                stack.push(char);
            }
        }
        let mut new_stack = vec![];
        for char in stack {
            if !new_stack.is_empty() && new_stack.last() == Some(&'b') && char == 'a' {
                new_stack.pop();
                max += y;
            } else {
                new_stack.push(char);
            }
        }
    } else {
        for char in s.chars() {
            if !stack.is_empty() && stack.last() == Some(&'b') && char == 'a' {
                stack.pop();
                max += y;
            } else {
                stack.push(char);
            }
        }
        let mut new_stack = vec![];
        for char in stack {
            if !new_stack.is_empty() && new_stack.last() == Some(&'a') && char == 'b' {
                new_stack.pop();
                max += x;
            } else {
                new_stack.push(char);
            }
        }
    }


    max
}