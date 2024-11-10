/*You are given two integers n and t. Return the smallest number greater than or equal to n such that the product of its digits is divisible by t.



Example 1:

Input: n = 10, t = 2

Output: 10

Explanation:

The digit product of 10 is 0, which is divisible by 2, making it the smallest number greater than or equal to 10 that satisfies the condition.

Example 2:

Input: n = 15, t = 3

Output: 16

Explanation:

The digit product of 16 is 6, which is divisible by 3, making it the smallest number greater than or equal to 15 that satisfies the condition.



Constraints:

1 <= n <= 100
1 <= t <= 10

Note: Please do not copy the description during the contest to maintain the integrity of your submissions.*/
pub fn smallest_number(n: i32, t: i32) -> i32 {
    let mut product = 1;
    let mut temp = n;
    while temp != 0 {
        product *= temp % 10;
        temp /= 10;
    }

    if product % t == 0 {
        n
    } else {
        n + (t - n % t).min(10 - n % 10)
    }
}