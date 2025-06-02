derive!(Debug);
struct BIT {
    tree: Vec<i32>,
    n: usize,
}


impl BIT {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
            n: n + 1,
        }
    }

    pub fn update(&mut self, i: usize, val: i32) {
        let mut i = i + 1;
        while i < self.n {
            self.tree[i] += val;
            i += i & (!i + 1);;
        }
    }


    pub fn query(&self, i: usize) -> i32 {
        let mut i = i + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & (!i + 1);
        }
        sum
    }

    pub fn range_query(&self, left: usize, right: usize) -> i32 {
        if left == 0 {
            self.query(right)
        } else {
            self.query(right) - self.query(left - 1)
        }
    }
}