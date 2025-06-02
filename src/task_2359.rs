/*You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.

The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from i, then edges[i] == -1.

You are also given two integers node1 and node2.

Return the index of the node that can be reached from both node1 and node2, such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized. If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.

Note that edges may contain cycles.



Example 1:

Input: edges = [2,2,3,-1], node1 = 0, node2 = 1
Output: 2
Explanation: The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.

Example 2:

Input: edges = [1,2,-1], node1 = 0, node2 = 2
Output: 2
Explanation: The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.



Constraints:

n == edges.length
2 <= n <= 105
-1 <= edges[i] < n
edges[i] != i
0 <= node1, node2 < n

*/


pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {

    let n = edges.len();
    let node1 = node1 as usize;
    let node2 = node2 as usize;
    fn dfs(node: usize, edges: &Vec<i32>, memo: &mut Vec<i32>, depth: i32) {
        if memo[node] != -1 {
            return;
        }

        memo[node] = depth;

        if edges[node] == -1 {
            return;
        }
        dfs(edges[node] as usize, edges, memo, depth + 1);
    }

    let mut node1_nodes = vec![-1; n];
    dfs(node1, &edges, &mut node1_nodes, 0);
    let mut node2_nodes = vec![-1; n];
    dfs(node2, &edges, &mut node2_nodes, 0);



    let mut result = -1;
    let mut min = i32::MAX;
    for i in 0..n {
        if node1_nodes[i] == -1 || node2_nodes[i] == -1 {
            continue;
        }
        if min > node1_nodes[i].max(node2_nodes[i]) {
            min = node1_nodes[i].min(node2_nodes[i]);
            result = i as i32;
        }
    }
    result
}