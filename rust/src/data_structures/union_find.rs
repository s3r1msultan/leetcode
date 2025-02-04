struct UnionFind {
    representatives: Vec<usize>,
    // here could be ranks and other fields you need
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            representatives: vec![]
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let parent_a = self.find(a);
        let parent_b = self.find(b);

        if parent_a == parent_b {
            return;
        }

        Self.representatives[parent_b] = parent_a;
    }

    pub fn add(&mut self, a: usize, b: usize) {
        self.representatives[b] = a;
    }

    pub fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if parent == x {
            return x;
        }
        let root = self.find(parent);
        self.representatives[x] = root;
        root
    }
}

fn main() {
    let mut parents = vec![];
}