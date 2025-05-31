struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(arr: &Vec<i32>) -> Self {
        let n = arr.len();
        let tree = vec![0; 4 * n];
        let mut segment_tree = SegmentTree { n, tree };
        segment_tree.build(1, 0, n - 1, arr);
        segment_tree
    }

    fn build(&mut self, i: usize, left: usize, right: usize, arr: &Vec<i32>) {
        if left == right {
            self.tree[i] = arr[left];
        } else {
            let mid = left + (right - left) / 2;
            self.build(2 * i, left, mid, arr);
            self.build(2 * i + 1, mid + 1, right, arr);
            self.tree[i] = self.tree[2 * i] + self.tree[2 * i + 1];
        }
    }

    fn query(&self, left: usize, right: usize) -> i32 {
        self.tree_query(1, 0, self.n - 1, left, right)
    }

    fn tree_query(&self, i: usize, tree_left: usize, tree_right: usize, left: usize, right: usize) -> i32 {
        if left == tree_left && right == tree_right {
            self.tree[i]
        } else {
            let tree_mid = tree_left + (tree_right - tree_left) / 2;
            self.tree_query(2 * i, tree_left, tree_mid, left, right.min(tree_mid)) + self.tree_query(2 * i + 1, tree_mid + 1, tree_right, left.max(tree_mid), right)
        }
    }

    fn update(&mut self, i: usize, x: i32) {
        self.tree_update(1, 0, self.n - 1, i, x);
    }

    fn tree_update(&mut self, tree_i: usize, tree_left: usize, tree_right: usize, i: usize, x: i32) {
        if tree_left == tree_right {
            self.tree[tree_i] = x;
        } else {
            let tree_mid = tree_left + (tree_right - tree_left) / 2;
            if i <= tree_mid {
                self.tree_update(tree_i * 2, tree_left, tree_mid, i, x);
            } else {
                self.tree_update(tree_i * 2 + 1, tree_mid + 1, tree_right, i, x);
            }
            self.tree[tree_i] = self.tree[tree_i * 2] + self.tree[tree_i * 2 + 1];
        }
    }
}