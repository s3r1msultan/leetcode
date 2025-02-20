/*There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.

You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.

Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.



Example 1:

Input: n = 4, connections = [[0,1],[0,2],[1,2]]
Output: 1
Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.

Example 2:

Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
Output: 2

Example 3:

Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
Output: -1
Explanation: There are not enough cables.



Constraints:

1 <= n <= 105
1 <= connections.length <= min(n * (n - 1) / 2, 105)
connections[i].length == 2
0 <= ai, bi < n
ai != bi
There are no repeated connections.
No two computers are connected by more than one cable.

*/

struct DS {
    representatives: Vec<usize>,
    ranks: Vec<i32>,
}

impl DS {
    fn new(n: usize) -> Self {
        Self {
            ranks: vec![1; n],
            representatives: (0..n).collect(),
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

        if self.ranks[root_a] > self.ranks[root_b] {
            self.representatives[root_b] = root_a;
            self.ranks[root_a] += 1;
        } else {
            self.representatives[root_a] = root_b;
            self.ranks[root_b] += 1;
        }
    }

    fn get_number_of_groups(&mut self) -> i32 {
        self.representatives.iter().enumerate().filter(|&(i, &x)| i == x).count() as i32
    }
}

pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    if connections.len() + 1 < n {
        return -1;
    }

    let mut set = DS::new(n);

    for connection in connections {
        let a = connection[0] as usize;
        let b = connection[1] as usize;

        set.union(a, b);
    }
    set.get_number_of_groups()
}