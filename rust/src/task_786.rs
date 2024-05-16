// You are given a sorted integer array arr containing 1 and prime numbers, where all the integers of arr are unique. You are also given an integer k.
//
// For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
//
// Return the kth smallest fraction considered. Return your answer as an array of integers of size 2, where answer[0] == arr[i] and answer[1] == arr[j].
//
//
//
// Example 1:
//
// Input: arr = [1,2,3,5], k = 3
// Output: [2,5]
// Explanation: The fractions to be considered in sorted order are:
// 1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
// The third fraction is 2/5.
//
// Example 2:
//
// Input: arr = [1,7], k = 1
// Output: [1,7]
//
//
//
// Constraints:
//
// 2 <= arr.length <= 1000
// 1 <= arr[i] <= 3 * 104
// arr[0] == 1
// arr[i] is a prime number for i > 0.
// All the numbers of arr are unique and sorted in strictly increasing order.
// 1 <= k <= arr.length * (arr.length - 1) / 2


// pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
// 	let mut fractions:Vec<(f32, i32, i32)> = vec![];
// 	for i in 0..arr.len() {
// 		for j in i+1..arr.len() {
// 			fractions.push((arr[i] as f32/arr[j] as f32, arr[i], arr[j]))
// 		}
// 	}
// 	fractions.sort_unstable_by(|&a,&b| a.0.total_cmp(&b.0));
// 	let result: Vec<i32> = vec![fractions[k as usize - 1].1, fractions[k as usize - 1].2];
// 	return result;
// }

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
struct Fraction {
	numerator: i32,
	denominator: i32,
}

impl Ord for Fraction {
	fn cmp(&self, other: &Self) -> Ordering {
		let self_mult = self.numerator as i64 * other.denominator as i64;
		let other_mult = other.numerator as i64 * self.denominator as i64;
		self_mult.cmp(&other_mult)
	}
}

impl PartialOrd for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
	let n = arr.len();
	let mut heap = BinaryHeap::new();

	for j in 1..n {
		heap.push(Reverse(Fraction {
			numerator: arr[0],
			denominator: arr[j],
		}));
	}

	for _ in 0..k-1 {
		let smallest = heap.pop().unwrap().0;
		let i = arr.iter().position(|&x| x == smallest.numerator).unwrap();
		if i + 1 < n {
			heap.push(Reverse(Fraction {
				numerator: arr[i + 1],
				denominator: smallest.denominator,
			}));
		}
	}

	let kth_fraction = heap.pop().unwrap().0;
	vec![kth_fraction.numerator, kth_fraction.denominator]
}
#[cfg(test)]

#[test]

fn test_kth_smallest_prime_fraction() {
	let arr = vec![1,2,3,5];
	let k = 3;
	let result = vec![2,5];
	assert_eq!(kth_smallest_prime_fraction(arr, k), result);

	let arr = vec![1,7];
	let k = 1;
	let result = vec![1,7];
	assert_eq!(kth_smallest_prime_fraction(arr, k), result);
}