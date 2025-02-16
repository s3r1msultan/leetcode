/*You are given a positive integer n representing the number of nodes in an undirected graph. The nodes are labeled from 1 to n.

You are also given a 2D integer array edges, where edges[i] = [ai, bi] indicates that there is a bidirectional edge between nodes ai and bi. Notice that the given graph may be disconnected.

Divide the nodes of the graph into m groups (1-indexed) such that:

Each node in the graph belongs to exactly one group.
For every pair of nodes in the graph that are connected by an edge [ai, bi], if ai belongs to the group with index x, and bi belongs to the group with index y, then |y - x| = 1.

Return the maximum number of groups (i.e., maximum m) into which you can divide the nodes. Return -1 if it is impossible to group the nodes with the given conditions.



Example 1:

Input: n = 6, edges = [[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]]
Output: 4
Explanation: As shown in the image we:
- Add node 5 to the first group.
- Add node 1 to the second group.
- Add nodes 2 and 4 to the third group.
- Add nodes 3 and 6 to the fourth group.
We can see that every edge is satisfied.
It can be shown that that if we create a fifth group and move any node from the third or fourth group to it, at least on of the edges will not be satisfied.

Example 2:

Input: n = 3, edges = [[1,2],[2,3],[3,1]]
Output: -1
Explanation: If we add node 1 to the first group, node 2 to the second group, and node 3 to the third group to satisfy the first two edges, we can see that the third edge will not be satisfied.
It can be shown that no grouping is possible.



Constraints:

1 <= n <= 500
1 <= edges.length <= 104
edges[i].length == 2
1 <= ai, bi <= n
ai != bi
There is at most one edge between any pair of vertices.

*/


use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

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

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
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
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn bfs(node: usize, graph: &Vec<Vec<usize>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut depth: HashMap<usize, i32> = HashMap::new();

    queue.push_back(node);
    depth.insert(node, 1);

    let mut max_depth = 1;

    while let Some(node) = queue.pop_front() {
        let curr_depth = depth.get(*node).unwrap().clone();

        for neighbour in graph[node] {
            if depth.contains_key(&neighbour) {
                if (depth[&neighbour] - curr_depth).abs() != 1 {
                    return -1;
                }
            } else {
                depth.insert(neighbour, curr_depth + 1);
                max_depth = max_depth.max(curr_depth + 1);
                queue.push_back(neighbour);
            }
        }
    }

    max_depth
}

struct Solution;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut set = DS::new(n + 1);
        let mut graph = vec![vec![]; n + 1];

        for i in 0..edges.len() {
            let u = edges[i][0] as usize;
            let v = edges[i][1] as usize;
            set.union(u, v);
            graph[u].push(v);
        }

        let mut components = HashMap::new();
        for i in 1..n + 1 {
            let root = set.find(i);
            components.entry(root).or_insert_with(Vec::new).push(i);
        }

        let mut result = 0;

        for (_, group) in components {
            let mut max_depth = -1;

            for node in group {
                let curr_depth = bfs(node, &graph);
                if curr_depth == -1 {
                    return -1;
                }
                max_depth = max_depth.max(curr_depth);
            }
            result += max_depth;
        }

        result
    }
}