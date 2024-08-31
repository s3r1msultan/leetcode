/*Given an integer n, return the number of prime numbers that are strictly less than n.



Example 1:

Input: n = 10
Output: 4
Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.

Example 2:

Input: n = 0
Output: 0

Example 3:

Input: n = 1
Output: 0



Constraints:

0 <= n <= 5 * 106

*/

fn count_primes(n: i32) -> i32 {
	if n <= 1 {
		return 0;
	}
	let mut n = n;
	let mut count = 0;
	for i in 2..=(n as f64).sqrt() as i32 {
		if n % i == 0 {
			while n % i != 0 {}
		}
	}

	count
}