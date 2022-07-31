// BIT tree
// BIT tree
struct BIT {
    tree: Vec<i32>,
}

impl BIT {
    fn new(size: usize) -> Self {
        let tree = vec![0; size + 1];
        Self {
            tree,
        }
    }

    fn sum(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut index = (index + 1) as i32;
        while index > 0 {
            sum += self.tree[index as usize];
            index -= index & (-index);
        }
        sum
    }

    fn update(&mut self, index: usize, val: i32) {
        let n = (self.tree.len() - 1) as i32;
        let mut index = (index + 1) as i32;
        while index <= n {
            self.tree[index as usize] += val;
            index += index & (-index);
        }
    }
}
