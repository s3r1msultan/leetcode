/*There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.

A province is a group of directly or indirectly connected cities and no other cities outside of the group.

You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.

Return the total number of provinces.



Example 1:

Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
Output: 2

Example 2:

Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
Output: 3



Constraints:

1 <= n <= 200
n == isConnected.length
n == isConnected[i].length
isConnected[i][j] is 1 or 0.
isConnected[i][i] == 1
isConnected[i][j] == isConnected[j][i]

*/
// DFS Solution
// fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
// 	for row in is_connected.iter() {
// 		println!("{row:?}");
// 	}
// 	fn dfs(visited_nodes: &mut Vec<bool>, node: usize, is_connected: &Vec<Vec<i32>>) {
// 		let n = is_connected.len();
// 		visited_nodes[node] = true;
// 		for i in 0..node {
// 			if is_connected[node][i] == 1 && !visited_nodes[i] {
// 				dfs(visited_nodes, i, is_connected);
// 			}
// 		}
// 	}
// 	let n = is_connected.len();
// 	let mut visited_nodes = vec![false; n];
// 	let mut count = 0;
// 	for i in 0..n {
// 		if !visited_nodes[i] {
// 			count += 1;
// 			dfs(&mut visited_nodes, i, &is_connected);
// 		}
// 	}
// 	println!();
// 	for row in is_connected.iter() {
// 		println!("{row:?}");
// 	}
// 	count
// }