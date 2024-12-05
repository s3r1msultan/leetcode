<<<<<<< HEAD
/*There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
=======
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
1466. Reorder Routes to Make All Paths Lead to the City Zero
Medium
Topics
Companies
Hint

There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
>>>>>>> 2276d2c53b5e5f30758ccd90b506909ec90665d7

Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.

This year, there will be a big event in the capital (city 0), and many people want to travel to this city.

Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.

It's guaranteed that each city can reach city 0 after reorder.



Example 1:

Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
Output: 3
Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).

Example 2:

Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
Output: 2
Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).

Example 3:

Input: n = 3, connections = [[1,0],[2,0]]
Output: 0



Constraints:

2 <= n <= 5 * 104
connections.length == n - 1
connections[i].length == 2
0 <= ai, bi <= n - 1
ai != bi

*/

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
<<<<<<< HEAD
    let n = n as usize;
    let mut adj = vec![vec![]; n];

    for connection in connections {
        let u = connection[0] as usize;
        let v = connection[1] as usize;
        adj[u].push((v, true));
        adj[v].push((u, false));
    }

    let mut visited = vec![false; n];
    let mut stack = vec![];
    let mut count = 0;

    stack.push(0);
    visited[0] = true;

    while let Some(city) = stack.pop() {
        visited[n] = true;
        for &(n, is_forward) in &adj[city] {
            if visited[n] {
                continue;
            }
            if is_forward { count += 1; }
            stack.push(n);
=======
    let mut count = 0;
    let mut stack = vec![];
    let mut visited = vec![false; n as usize];
    for i in 0..n {
        if visited[i] {
            continue;
>>>>>>> 2276d2c53b5e5f30758ccd90b506909ec90665d7
        }
    }

    count
}