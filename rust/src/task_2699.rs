/*Editorial
Editorial
Solutions
Solutions
Submissions
Submissions
Code
Testcase
Test Result
Test Result
2699. Modify Graph Edge Weights
Hard
Topics
Companies
Hint

You are given an undirected weighted connected graph containing n nodes labeled from 0 to n - 1, and an integer array edges where edges[i] = [ai, bi, wi] indicates that there is an edge between nodes ai and bi with weight wi.

Some edges have a weight of -1 (wi = -1), while others have a positive weight (wi > 0).

Your task is to modify all edges with a weight of -1 by assigning them positive integer values in the range [1, 2 * 109] so that the shortest distance between the nodes source and destination becomes equal to an integer target. If there are multiple modifications that make the shortest distance between source and destination equal to target, any of them will be considered correct.

Return an array containing all edges (even unmodified ones) in any order if it is possible to make the shortest distance from source to destination equal to target, or an empty array if it's impossible.

Note: You are not allowed to modify the weights of edges with initial positive weights.



Example 1:

Input: n = 5, edges = [[4,1,-1],[2,0,-1],[0,3,-1],[4,3,-1]], source = 0, destination = 1, target = 5
Output: [[4,1,1],[2,0,1],[0,3,3],[4,3,1]]
Explanation: The graph above shows a possible modification to the edges, making the distance from 0 to 1 equal to 5.

Example 2:

Input: n = 3, edges = [[0,1,-1],[0,2,5]], source = 0, destination = 2, target = 6
Output: []
Explanation: The graph above contains the initial edges. It is not possible to make the distance from 0 to 2 equal to 6 by modifying the edge with weight -1. So, an empty array is returned.

Example 3:

Input: n = 4, edges = [[1,0,4],[1,2,3],[2,3,5],[0,3,-1]], source = 0, destination = 2, target = 6
Output: [[1,0,4],[1,2,3],[2,3,5],[0,3,1]]
Explanation: The graph above shows a modified graph having the shortest distance from 0 to 2 as 6.



Constraints:

1 <= n <= 100
1 <= edges.length <= n * (n - 1) / 2
edges[i].length == 3
0 <= ai, bi < n
wi = -1 or 1 <= wi <= 107
ai != bi
0 <= source, destination < n
source != destination
1 <= target <= 109
The graph is connected, and there are no self-loops or repeated edges

*/

fn modified_graph_edges(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
    let mut graph = vec![vec![]; n as usize];
    for edge in edges.iter() {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        if w == -1 {
            graph[u].push((v, 0));
            graph[v].push((u, 0));
        } else {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
    }
    let mut dist = vec![std::i32::MAX; n as usize];
    dist[source as usize] = 0;
    let mut pq = std::collections::BinaryHeap::new();
    pq.push((0, source));
    while let Some((d, u)) = pq.pop() {
        if d > dist[u as usize] {
            continue;
        }
        for &(v, w) in graph[u].iter() {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                pq.push((dist[v], v));
            }
        }
    }
    let mut res = vec![];
    for edge in edges.iter() {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        if w == -1 {
            if dist[u] + 1 + dist[v] == dist[destination as usize] {
                res.push(vec![u as i32, v as i32, 1]);
            } else if dist[u] + 2 + dist[v] == dist[destination as usize] {
                res.push(vec![u as i32, v as i32, 2]);
            }
        } else {
            res.push(vec![u, v, w]);
        }
    }
    if dist[destination as usize] == target {
        res
    } else {
        vec![]
    }
}