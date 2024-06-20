/*You are given a positive integer num consisting only of digits 6 and 9.

Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).



Example 1:

Input: num = 9669
Output: 9969
Explanation:
Changing the first digit results in 6669.
Changing the second digit results in 9969.
Changing the third digit results in 9699.
Changing the fourth digit results in 9666.
The maximum number is 9969.

Example 2:

Input: num = 9996
Output: 9999
Explanation: Changing the last digit 6 to 9 results in the maximum number.

Example 3:

Input: num = 9999
Output: 9999
Explanation: It is better not to apply any change.



Constraints:

1 <= num <= 104
num consists of only 6 and 9 digits.

*/


fn maximum69_number(n: i32) -> i32 {
    let mut n = n;
    let mut new_num = 0;
    while n != 0 {
        new_num *= 10;
        new_num += n % 10;
        n /= 10;
    }
    let mut is_replaced = false;

    while new_num != 0 {
        n *= 10;
        if !is_replaced && new_num % 10 == 6 {
            n += 9;
            is_replaced = true;
        } else {
            n += new_num % 10;
        }
        new_num /= 10;
    }
    n
}

#[cfg(test)]
#[test]
fn test_maximum69_number() {
    let n = 9669;
    let result = 9969;
    assert_eq!(maximum69_number(n), result);

    let n = 9996;
    let result = 9999;
    assert_eq!(maximum69_number(n), result);

    let n = 9999;
    let result = 9999;
    assert_eq!(maximum69_number(n), result);

    let n = 669;
    let result = 969;
    assert_eq!(maximum69_number(n), result);
}