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


struct Solution;


impl Solution {
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

        let mut min_cost = 0;

        costs.sort();

        let mut union_find = UnionFind::new(n);

        for (i, j, cost) in costs {
            if !union_find.union(i, j) {
                min_cost += cost;
            }
        }

        min_cost
    }
}