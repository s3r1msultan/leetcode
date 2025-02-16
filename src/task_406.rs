/*You are given an array of people, people, which are the attributes of some people in a queue (not necessarily in order). Each people[i] = [hi, ki] represents the ith person of height hi with exactly ki other people in front who have a height greater than or equal to hi.

Reconstruct and return the queue that is represented by the input array people. The returned queue should be formatted as an array queue, where queue[j] = [hj, kj] is the attributes of the jth person in the queue (queue[0] is the person at the front of the queue).



Example 1:

Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
Output: [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
Explanation:
Person 0 has height 5 with no other people taller or the same height in front.
Person 1 has height 7 with no other people taller or the same height in front.
Person 2 has height 5 with two persons taller or the same height in front, which is person 0 and 1.
Person 3 has height 6 with one person taller or the same height in front, which is person 1.
Person 4 has height 4 with four people taller or the same height in front, which are people 0, 1, 2, and 3.
Person 5 has height 7 with one person taller or the same height in front, which is person 1.
Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.

Example 2:

Input: people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
Output: [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]



Constraints:

1 <= people.length <= 2000
0 <= hi <= 106
0 <= ki < people.length
It is guaranteed that the queue can be reconstructed.

*/

struct SegmentTree {
    tree: Vec<usize>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut arr = vec![0; n * 4];
        let mut segment_tree = Self { n, tree: arr };
        segment_tree.build(0, 0, n - 1);
        segment_tree
    }

    fn build(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = 1;
            return;
        }

        let mid = start + (end - start) / 2;
        let left_child = node * 2 + 1;
        let right_child = node * 2 + 2;

        self.build(left_child, start, mid);
        self.build(right_child, mid + 1, end);

        self.tree[node] = self.tree[left_child] + self.tree[right_child];
    }


    fn update(&mut self, index: usize) {
        self.update_recursive(0, 0, self.n - 1, index)
    }

    fn update_recursive(&mut self, node: usize, start: usize, end: usize, index: usize) {
        if start == end {
            self.tree[node] = 0;
        } else {
            let mid = start + (end - start) / 2;
            let left_child = node * 2 + 1;
            let right_child = node * 2 + 2;

            if index <= mid {
                self.update_recursive(left_child, start, mid, index);
            } else {
                self.update_recursive(right_child, mid + 1, end, index);
            }

            self.tree[node] = self.tree[left_child] + self.tree[right_child];
        }
    }

    fn query(&self, k: usize) -> usize {
        self.query_recursive(0, 0, self.n - 1, k + 1)
    }

    fn query_recursive(&self, node: usize, start: usize, end: usize, k: usize) -> usize {
        if start == end {
            start
        } else {
            let mid = start + (end - start) / 2;
            let left_child = node * 2 + 1;
            let right_child = node * 2 + 2;

            if self.tree[left_child] >= k {
                self.query_recursive(left_child, start, mid, k)
            } else {
                self.query_recursive(right_child, mid + 1, end, k - self.tree[left_child])
            }
        }
    }
}

pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = people.len();
    let mut people = people;
    people.sort_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            b[0].cmp(&a[0])
        }
    });

    println!("{:?}", people);
    let mut segment_tree = SegmentTree::new(n);
    let mut result = vec![vec![0, 0]; n];

    for i in 0..n {
        let h = people[i][0];
        let k = people[i][1];

        let pos = segment_tree.query(k as usize);

        result[pos] = vec![h, k];

        segment_tree.update(pos);
    }

    result
}