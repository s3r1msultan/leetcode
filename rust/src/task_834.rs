// There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.
//
// You are given the integer n and the array edges where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// Return an array answer of length n where answer[i] is the sum of the distances between the ith node in the tree and all other nodes.
//
//
//
// Example 1:
//
// Input: n = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
// Output: [8,12,6,10,10,10]
// Explanation: The tree is shown above.
// We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
// equals 1 + 1 + 2 + 2 + 2 = 8.
// Hence, answer[0] = 8, and so on.
//
// Example 2:
//
// Input: n = 1, edges = []
// Output: [0]
//
// Example 3:
//
// Input: n = 2, edges = [[1,0]]
// Output: [1,1]
//
//
//
// Constraints:
//
// 1 <= n <= 3 * 104
// edges.length == n - 1
// edges[i].length == 2
// 0 <= ai, bi < n
// ai != bi
// The given input represents a valid tree.
//


use std::collections::HashMap;

pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut count: Vec<i32> = vec![0; n as usize];
    let mut res: Vec<i32> = vec![0; n as usize];

    for edge in edges {
        let u = edge[0];
        let v = edge[1];
        graph.entry(u).or_insert(vec![]).push(v);
        graph.entry(v).or_insert(vec![]).push(u);
    }

    fn dfs1(cur: i32, parent: i32, graph: &HashMap<i32, Vec<i32>>, count: &mut Vec<i32>, res: &mut Vec<i32>) {
        count[cur as usize] = 1;
        if let Some(children) = graph.get(&cur) {
            for &child in children {
                if child != parent {
                    dfs1(child, cur, graph, count, res);
                    count[cur as usize] += count[child as usize];
                    res[cur as usize] += res[child as usize] + count[child as usize];
                }
            }
        }
    }

    fn dfs2(cur:i32, parent:i32, n:i32, graph: &HashMap<i32, Vec<i32>>, count: &Vec<i32>, res: &mut Vec<i32>) {
        if let Some(children) = graph.get(&cur) {
            for &child in children {
                if child != parent {
                    res[child as usize] = res[cur as usize] + n - 2*count[child as usize];
                    dfs2(child, cur, n, graph, count, res);
                }
            }
        }
    }

    dfs1(0, -1, &graph, &mut count, &mut res);
    dfs2(0, -1, n, &graph, &count, &mut res);


    res
}























