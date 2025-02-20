// There are n workers. You are given two integer arrays quality and wage where quality[i] is the quality of the ith worker and wage[i] is the minimum wage expectation for the ith worker.
//
// We want to hire exactly k workers to form a paid group. To hire a group of k workers, we must pay them according to the following rules:
//
// Every worker in the paid group must be paid at least their minimum wage expectation.
// In the group, each worker's pay must be directly proportional to their quality. This means if a worker’s quality is double that of another worker in the group, then they must be paid twice as much as the other worker.
//
// Given the integer k, return the least amount of money needed to form a paid group satisfying the above conditions. Answers within 10-5 of the actual answer will be accepted.
//
//
//
// Example 1:
//
// Input: quality = [10,20,5], wage = [70,50,30], k = 2
// Output: 105.00000
// Explanation: We pay 70 to 0th worker and 35 to 2nd worker.
//
// Example 2:
//
// Input: quality = [3,1,10,10,1], wage = [4,8,2,2,7], k = 3
// Output: 30.66667
// Explanation: We pay 4 to 0th worker, 13.33333 to 2nd and 3rd workers separately.
//
//
//
// Constraints:
//
// n == quality.length == wage.length
// 1 <= k <= n <= 104
// 1 <= quality[i], wage[i] <= 104
//

use std::collections::BinaryHeap;
use std::fmt::Debug;

#[derive(Debug)]
struct Worker {
	wage: i32,
	quality: i32,
	ratio: f64,
}

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
	let k = k as usize;
	let mut workers: Vec<Worker> = vec![];
	for (&q, w) in quality.iter().zip(wage) {
		workers.push(Worker{wage: w, quality: q, ratio: w as f64/q as f64});
	}
	workers.sort_by(|a,b| a.ratio.total_cmp(&b.ratio));

	let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
	let mut min_cost = std::f64::MAX;
	let mut sum_quality = 0;
	println!("{:#?}", workers);
	for worker in workers {
		max_heap.push(worker.quality);
		println!("{:#?}", max_heap);
		sum_quality += worker.quality;
		if max_heap.len() > k {
			sum_quality -= max_heap.pop().unwrap();
		}
		if max_heap.len() == k {
			let cost = sum_quality as f64 * worker.ratio;
			min_cost = min_cost.min(cost);
		}
	}
	format!("{:.5}", min_cost).parse::<f64>().unwrap()
}

#[cfg(test)]

#[test]

fn test_mincost_to_hire_workers() {
	let quality = vec![10,20,5];
	let wage = vec![70,50,30];
	let k = 2;
	let result = 105.0;
	assert_eq!(mincost_to_hire_workers(quality, wage, k), result);

	let quality = vec![3,1,10,10,1];
	let wage = vec![4,8,2,2,7];
	let k = 3;
	let result = 30.66667;
	assert_eq!(mincost_to_hire_workers(quality, wage, k), result);
}