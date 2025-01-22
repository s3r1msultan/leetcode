/*You are given an array points representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].

The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.

Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.



Example 1:

Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
Output: 20
Explanation:

We can connect the points as shown above to get the minimum cost of 20.
Notice that there is a unique path between every pair of points.

Example 2:

Input: points = [[3,12],[-2,5],[-4,1]]
Output: 18



Constraints:

1 <= points.length <= 1000
-106 <= xi, yi <= 106
All pairs (xi, yi) are distinct.

*/
use std::cmp::Ordering;

struct UnionFind {
    representatives: Vec<usize>,
    ranks: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            ranks: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if parent == x {
            return x;
        }
        let root = self.find(parent);
        self.representatives[x] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return false;
        }

        match self.ranks[root_a].cmp(&self.ranks[root_b]) {
            Ordering::Less => {
                self.representatives[root_a] = root_b;
            }
            Ordering::Equal => {
                self.representatives[root_b] = root_a;
                self.ranks[root_a] += 1;
            }
            Ordering::Greater => {
                self.representatives[root_b] = root_a;
            }
        }
        true
    }
}
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut costs = vec![];

    for i in 0..n {
        let a = &points[i];
        let x1 = a[0];
        let y1 = a[1];
        for j in i + 1..n {
            let b = &points[j];
            let x2 = b[0];
            let y2 = b[1];

            let cost = (x1 - x2).abs() + (y1 - y2).abs();

            costs.push((i, j, cost));
        }
    }

    costs.sort_by(|a, b| a.2.cmp(&b.2));

    let mut set = UnionFind::new(n);

    let mut result = 0;

    for (i, j, cost) in costs {
        if set.union(i, j) {
            result += cost;
        }
    }

    result
}