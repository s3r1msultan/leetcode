/*Given two positive integers left and right, find the two integers num1 and num2 such that:

left <= num1 < num2 <= right .
Both num1 and num2 are

.
num2 - num1 is the minimum amongst all other pairs satisfying the above conditions.

Return the positive integer array ans = [num1, num2]. If there are multiple pairs satisfying these conditions, return the one with the smallest num1 value. If no such numbers exist, return [-1, -1].



Example 1:

Input: left = 10, right = 19
Output: [11,13]
Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
Since 11 is smaller than 17, we return the first pair.

Example 2:

Input: left = 4, right = 6
Output: [-1,-1]
Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.



Constraints:

1 <= left <= right <= 106
*/

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let left = left as usize;
    let right = right as usize;
    let mut nums = vec![false; right + 1];
    let mut primes = vec![];

    for i in 2..=right {
        if !nums[i] {
            if i >= left {
                primes.push(i as i32) ;
            }
            for j in (i..=right).step_by(i) {
                nums[j] = true;
            }
        }
    }

    if primes.len() < 2 {
        return vec![-1, -1];
    }

    let mut result = vec![];
    let mut diff = i32::MAX;
    for i in 1..primes.len() {
        if diff > primes[i] - primes[i-1] {
            diff = primes[i] - primes[i-1];
            result[0] = primes[i-1];
            result[1] = primes[i];
        }
    }
    result
}