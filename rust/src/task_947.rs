/*On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.

A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.

Given an array stones of length n where stones[i] = [xi, yi] represents the location of the ith stone, return the largest possible number of stones that can be removed.



Example 1:

Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
Output: 5
Explanation: One way to remove 5 stones is as follows:
1. Remove stone [2,2] because it shares the same row as [2,1].
2. Remove stone [2,1] because it shares the same column as [0,1].
3. Remove stone [1,2] because it shares the same row as [1,0].
4. Remove stone [1,0] because it shares the same column as [0,0].
5. Remove stone [0,1] because it shares the same row as [0,0].
Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.

Example 2:

Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
Output: 3
Explanation: One way to make 3 moves is as follows:
1. Remove stone [2,2] because it shares the same row as [2,0].
2. Remove stone [2,0] because it shares the same column as [0,0].
3. Remove stone [0,2] because it shares the same row as [0,0].
Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.

Example 3:

Input: stones = [[0,0]]
Output: 0
Explanation: [0,0] is the only stone on the plane, so you cannot remove it.



Constraints:

1 <= stones.length <= 1000
0 <= xi, yi <= 104
No two stones are at the same coordinate point.

*/

#[derive(Clone, Debug)]
pub struct DSU {
	parent: Vec<usize>,
}

impl DSU {
	pub fn new(n: usize) -> Self {
		Self {
			parent: vec![usize::MAX; n],
		}
	}

	pub fn count(&self) -> usize {
		self.parent
			.iter()
			.enumerate()
			.map(|(i, &x)| (i == x) as usize)
			.sum::<usize>()
	}

	pub fn make_set(&mut self, v: usize) {
		if self.parent[v] == usize::MAX {
			self.parent[v] = v;
		}
	}

	pub fn find_set(&mut self, v: usize) -> usize {
		if v == self.parent[v] {
			v
		} else {
			self.parent[v] = self.find_set(self.parent[v]);
			self.parent[v]
		}
	}

	pub fn union_sets(&mut self, a: usize, b: usize) -> bool {
		let a = self.find_set(a);
		let b = self.find_set(b);
		if a != b {
			self.parent[b] = a;
			true
		} else {
			false
		}
	}
}
fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
	let mut dsu = DSU::new(20002);
	for x in &stones {
		let (r, c) = (x[0] as usize, 10001 + x[1] as usize);
		dsu.make_set(r);
		dsu.make_set(c);
		dsu.union_sets(r, c);
	}
	(stones.len() - dsu.count()) as i32
}