/*In this problem, a tree is an undirected graph that is connected and has no cycles.

You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.

Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.



Example 1:

Input: edges = [[1,2],[1,3],[2,3]]
Output: [2,3]

Example 2:

Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
Output: [1,4]



Constraints:

n == edges.length
3 <= n <= 1000
edges[i].length == 2
1 <= ai < bi <= edges.length
ai != bi
There are no repeated edges.
The given graph is connected.

*/
use std::cmp::Ordering;

struct DS {
    representatives: Vec<usize>,
    ranks: Vec<i32>,
}

impl DS {
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
            return true;
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

        false
    }
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut disjoint_set = DS::new(n);

    let mut redundant_connections = vec![];

    for edge in &edges {
        let a = edge[0] as usize - 1;
        let b = edge[1] as usize - 1;
        if disjoint_set.union(a, b) {
            redundant_connections.push(edge.clone());
        }
    }

    redundant_connections.last().unwrap().clone()
}