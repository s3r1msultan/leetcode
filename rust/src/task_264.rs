/*An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.

Given an integer n, return the nth ugly number.



Example 1:

Input: n = 10
Output: 12
Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.

Example 2:

Input: n = 1
Output: 1
Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.



Constraints:

1 <= n <= 1690

*/



use std::cmp::Reverse;

fn nth_ugly_number(n: i32) -> i32 {
	use std::cmp::Reverse;
	use std::collections::{BinaryHeap, HashSet};

	let prime_factors: [i64; 3] = [2, 3, 5];
	let mut min_heap = BinaryHeap::new();
	let mut seen_numbers = HashSet::new();

	min_heap.push(Reverse(1));
	seen_numbers.insert(1);

	let mut current_ugly = 1;
	for i in 0..n {
		current_ugly = min_heap.pop().unwrap().0;
		for prime in prime_factors {
			let next_ugly: i64 = current_ugly * prime;
			if !seen_numbers.contains(&next_ugly) {
				min_heap.push(Reverse(next_ugly));
				seen_numbers.insert(next_ugly);
			}
		}
	}

	current_ugly as i32
}