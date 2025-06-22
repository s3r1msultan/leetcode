/*There exist two undirected trees with n and m nodes, with distinct labels in ranges [0, n - 1] and [0, m - 1], respectively.

You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree. You are also given an integer k.

Node u is target to node v if the number of edges on the path from u to v is less than or equal to k. Note that a node is always target to itself.

Return an array of n integers answer, where answer[i] is the maximum possible number of nodes target to node i of the first tree if you have to connect one node from the first tree to another node in the second tree.

Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.



Example 1:

Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2

Output: [9,7,9,8,8]

Explanation:

For i = 0, connect node 0 from the first tree to node 0 from the second tree.
For i = 1, connect node 1 from the first tree to node 0 from the second tree.
For i = 2, connect node 2 from the first tree to node 4 from the second tree.
For i = 3, connect node 3 from the first tree to node 4 from the second tree.
For i = 4, connect node 4 from the first tree to node 4 from the second tree.

Example 2:

Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]], k = 1

Output: [6,3,3,3,3]

Explanation:

For every i, connect node i of the first tree with any node of the second tree.



Constraints:

2 <= n, m <= 1000
edges1.length == n - 1
edges2.length == m - 1
edges1[i].length == edges2[i].length == 2
edges1[i] = [ai, bi]
0 <= ai, bi < n
edges2[i] = [ui, vi]
0 <= ui, vi < m
The input is generated such that edges1 and edges2 represent valid trees.
0 <= k <= 1000

*/

pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    fn dfs(i: usize, map: &Vec<Vec<usize>>, k: i32, visited: &mut Vec<bool>) -> i32 {
        if k == 0 {
            return 0;
        }
        let mut count = 1;
        for vertex in map[i] {
            if visited[vertex] {
                continue;
            }
            visited[vertex] = true;
            count += dfs(vertex, map, k - 1, visited);
            visited[vertex] = false;
        }
        count
    }

    let n = edges1.len();
    let m = edges2.len();
    
    let mut map_1 = vec![vec![]; n];
    let mut map_2 = vec![vec![]; m];

    for edge in edges1 {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        map_1[u].push(v);
        map_1[v].push(u);
    }
    
    for edge in edges2 {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        map_2[u].push(v);
        map_2[v].push(u);
    }

    let mut values_1 = vec![0; n];
    let mut visited = vec![false; n];
    for i in 0..n {
        visited[i] = true;
        values_1[i] = dfs(i, &map_1, k, &mut visited);
        visited[i] = false;
    }

    let mut max_value_2 = 1;
    let mut visited = vec![false; m];
    for j in 0..m {
        visited[j] = true;
        max_value_2 = max_value_2.max(dfs(j, &map_2, k, &mut visited));
        visited[j] = false;
    }

    let mut result = vec![1; n];

    for i in 0..n {
        result[i] = values_1[i] + max_value_2;
    }

    result
}