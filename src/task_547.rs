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

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Ordering;

    struct UnionSet {
        representatives: Vec<i32>,
        rank: Vec<i32>,
        count: i32,
    }

    impl UnionSet {
        fn new(n: usize) -> Self {
            Self {
                representatives: (0..n as i32).collect(),
                rank: vec![1; n],
                count: n as i32,
            }
        }

        fn find(&mut self, x: i32) -> i32 {
            let parent = self.representatives[x as usize];
            if parent == x {
                return x;
            }
            let root = self.find(parent);
            self.representatives[x as usize] = root;

            root
        }

        fn union(&mut self, a: i32, b: i32) {
            let root_a = self.find(a) as usize;
            let root_b = self.find(b) as usize;

            if root_a == root_b {
                return;
            }
            self.count -= 1;

            match self.rank[root_a].cmp(&self.rank[root_b]) {
                Ordering::Less => {
                    self.representatives[root_a] = root_b as i32;
                }
                Ordering::Equal => {
                    self.representatives[root_b] = root_a as i32;
                    self.rank[root_a] += 1;
                }
                Ordering::Greater => {
                    self.representatives[root_b] = root_a as i32
                }
            }
        }

        fn get_count(&self) -> i32 {
            self.count
        }
    }

    let n = is_connected.len();
    let mut union_set = UnionSet::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if is_connected[i][j] == 1 {
                union_set.union(i as i32, j as i32);
            }
        }
    }

    union_set.get_count()
}

