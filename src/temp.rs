/*You are given an integer n. There is an undirected graph with n vertices, numbered from 0 to n - 1. You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting vertices ai and bi.

Return the number of complete connected components of the graph.

A connected component is a subgraph of a graph in which there exists a path between any two vertices, and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.

A connected component is said to be complete if there exists an edge between every pair of its vertices.



Example 1:

Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
Output: 3
Explanation: From the picture above, one can see that all of the components of this graph are complete.

Example 2:

Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
Output: 1
Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between every pair of two vertices. On the other hand, the component containing vertices 3, 4, and 5 is not complete since there is no edge between vertices 4 and 5. Thus, the number of complete components in this graph is 1.



Constraints:

1 <= n <= 50
0 <= edges.length <= n * (n - 1) / 2
edges[i].length == 2
0 <= ai, bi <= n - 1
ai != bi
There are no repeated edges.
*/

struct UnionFind {
    representatives: Vec<usize>,
    sizes: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            sizes: vec![1; n],
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

        if self.sizes[root_a] >= self.sizes[root_b] {
            self.representatives[root_b] = root_a;
            self.sizes[root_a] += self.sizes[root_b];
            self.sizes[root_b] = self.sizes[root_a];
        } else {
            self.representatives[root_a] = root_b;
            self.sizes[root_b] += self.sizes[root_a];
            self.sizes[root_a] = self.sizes[root_b];
        }
    }

    fn get_size_of(&self, x: usize) -> i32 {
        self.sizes[x]
    }

    fn get_single_edges(&self) -> i32 {
        self.representatives.iter().enumerate().filter(|&(i, &representative)| i == representative && self.sizes[i] == 1).count() as i32
    }
}

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut uf = UnionFind::new(n);
    let mut map = std::collections::HashMap::new();
    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        uf.union(u, v);
    }

    for edge in &edges {
        let u = edge[0] as usize;
        let edge = uf.find(u);
        *map.entry(edge).or_insert(0) += 1;
    }

    let mut count = uf.get_single_edges();

    for (edge, vertex_count) in map {
        let edge_count = uf.get_size_of(edge);
        if vertex_count == (edge_count * (edge_count - 1)) / 2 {
            count += 1;
        }
    }

    count
}
