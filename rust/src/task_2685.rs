use std::collections::HashMap;

struct DS {
    representatives: Vec<usize>,
    size: Vec<i32>,
}

impl DS {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if x == parent {
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
        if self.size[root_a] >= self.size[root_b] {
            self.representatives[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
            self.size[root_b] = 0;
        } else {
            self.representatives[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
            self.size[root_a] = 0;
        }

        true
    }


    fn get_size_of(&self, x: usize) -> i32 {
        self.size[x]
    }

    fn get_single_vertices(&self) -> i32 {
        self.representatives.iter().enumerate().filter(|&(i, &x)| i == x && self.size[i] == 1).count() as i32
    }
}

struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut set = DS::new(n);
        let mut map = HashMap::new();

        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            set.union(a, b);

            let root_a = set.find(a);
            *map.entry(root_a).or_insert(0) += 1;
        }

        let mut count = set.get_single_vertices();

        for (x, edges) in map {
            let vertices = set.get_size_of(x);
            if edges == vertices * (vertices - 1) / 2 {
                count += 1;
            }
        }

        count
    }
}