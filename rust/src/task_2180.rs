/*Given a positive integer num, return the number of positive integers less than or equal to num whose digit sums are even.

The digit sum of a positive integer is the sum of all its digits.



Example 1:

Input: num = 4
Output: 2
Explanation:
The only integers less than or equal to 4 whose digit sums are even are 2 and 4.

Example 2:

Input: num = 30
Output: 14
Explanation:
The 14 integers less than or equal to 30 whose digit sums are even are
2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26, and 28.



Constraints:

1 <= num <= 1000

*/

/*pub fn count_even(num: i32) -> i32 {
    let mut count = 0;

    for i in 0..=num {
        let mut sum = 0;
        let mut num = i;
        while num != 0 {
            sum += num % 10;
            num /= 10;
        }
        if sum % 2 == 0 {
            count += 1;
        }
    }

    count
}*/


pub fn count_even(num: i32) -> i32 {
    if num % 2 == 1 {
        return (num - 1) / 2;
    }
    let sum = num % 10 + (num / 10) % 10 + (num / 100) % 10 + (num / 1000) % 10;
    num / 2 - sum % 2
}