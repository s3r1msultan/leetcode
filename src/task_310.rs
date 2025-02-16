// A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.
//
// Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).
//
// Return a list of all MHTs' root labels. You can return the answer in any order.
//
// The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
//
//
//
// Example 1:
//
// Input: n = 4, edges = [[1,0],[1,2],[1,3]]
// Output: [1]
// Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
//
// Example 2:
//
// Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
// Output: [3,4]
//
//
//
// Constraints:
//
// 1 <= n <= 2 * 104
// edges.length == n - 1
// 0 <= ai, bi < n
// ai != bi
// All the pairs (ai, bi) are distinct.
// The given input is guaranteed to be a tree and there will be no repeated edges.
//


use std::collections::{HashMap, HashSet, VecDeque};

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }

    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for pair in edges.iter() {
        let current_vertex = pair[0];
        let neighbour_vertex = pair[1];
        graph.entry(current_vertex).or_insert_with(HashSet::new).insert(neighbour_vertex);
        graph.entry(neighbour_vertex).or_insert_with(HashSet::new).insert(current_vertex);
    }

    let mut leaves: VecDeque<i32> = VecDeque::new();
    for (current_vertex, neighbour_vertices) in graph.iter() {
        if neighbour_vertices.len() == 1 {
            leaves.push_back(*current_vertex);
        }
    }

    let mut remaining_nodes = n;
    while remaining_nodes > 2 {
        let leaves_count = leaves.len();
        remaining_nodes -= leaves_count as i32;

        for _ in 0..leaves_count {
            let leaf = leaves.pop_front().unwrap();
            let &neigbor = graph.get(&leaf).unwrap().iter().next().unwrap();

            graph.get_mut(&neigbor).unwrap().remove(&leaf);
            if graph.get(&neigbor).unwrap().len() == 1 {
                leaves.push_back(neigbor);
            }
        }
    }

    leaves.iter().copied().collect()
}