/*You are given a positive integer n. Each digit of n has a sign according to the following rules:

The most significant digit is assigned a positive sign.
Each other digit has an opposite sign to its adjacent digits.

Return the sum of all digits with their corresponding sign.



Example 1:

Input: n = 521
Output: 4
Explanation: (+5) + (-2) + (+1) = 4.

Example 2:

Input: n = 111
Output: 1
Explanation: (+1) + (-1) + (+1) = 1.

Example 3:

Input: n = 886996
Output: 0
Explanation: (+8) + (-8) + (+6) + (-9) + (+9) + (-6) = 0.



Constraints:

1 <= n <= 109
*/

pub fn alternate_digit_sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut k = 0;
    let mut n = n;
    let mut factor = -1;
    while n != 0 {
        sum += factor * n % 10;
        factor *= -1;
        n /= 10;
        k += 1;
    }
    if k % 2 == 0 {
        -sum
    } else {
        sum
    }
}