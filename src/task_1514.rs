/*You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].

Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.

If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.



Example 1:

Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
Output: 0.25000
Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.

Example 2:

Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
Output: 0.30000

Example 3:

Input: n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
Output: 0.00000
Explanation: There is no path between 0 and 2.



Constraints:

2 <= n <= 10^4
0 <= start, end < n
start != end
0 <= a, b < n
a != b
0 <= succProb.length == edges.length <= 2*10^4
0 <= succProb[i] <= 1
There is at most one edge between every two nodes.

*/


use std::cmp::Ordering;

pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
	use std::collections::HashMap;
	use std::collections::BinaryHeap;
	use std::cmp::Ordering;
	struct Probability {
		prob: f64,
		node: i32,
	}
	impl Eq for Probability {}
	impl PartialEq<Self> for Probability {
		fn eq(&self, other: &Self) -> bool {
			self.prob == other.prob
		}
	}
	impl PartialOrd<Self> for Probability {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.prob.partial_cmp(&other.prob).unwrap().reverse())
		}
	}

	impl Ord for Probability {
		fn cmp(&self, other: &Self) -> Ordering {
			self.prob.partial_cmp(&other.prob).unwrap().reverse()
		}
	}

	let mut graph = HashMap::new();
	for i in 0..edges.len() {
		let edge = &edges[i];
		let prob = succ_prob[i];
		let probability_0 = Probability { prob, node: edge[0] };
		let probability_1 = Probability { prob, node: edge[1] };
		graph.entry(edge[0]).or_insert(vec![]).push(probability_1);
		graph.entry(edge[1]).or_insert(vec![]).push(probability_0);
	}
	let mut heap = BinaryHeap::new();
	let start_probability = Probability { prob: 0.0, node: start_node };
	heap.push(start_probability);
	let mut probs = vec![-1.0; n as usize];
	probs[start_node as usize] = 1.0;
	while let Some(Probability { prob, node }) = heap.pop() {
		if node == end_node {
			return prob;
		}
		if let Some(neighbors) = graph.get(&node) {
			for (Probability { node: neighbor, prob: edge_prob }) in neighbors {
				let new_prob = prob * edge_prob;
				if new_prob > probs[*neighbor as usize] {
					probs[*neighbor as usize] = new_prob;
					heap.push(Probability { prob: new_prob, node: *neighbor });
				}
			}
		}
	}
	0.0
}