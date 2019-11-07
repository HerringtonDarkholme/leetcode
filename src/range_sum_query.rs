struct NumArray {
    arr: Vec<i32>,
    nums: Vec<i32>,
}

#[inline]
fn lsb(i: i32) -> i32 {
    i & (-i)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut s = NumArray {
            arr: vec![0; n+1],
            nums: vec![0; n],
        };
        for i in 0..nums.len() {
            s.update(i as i32, nums[i]);
        }
        s
    }

    fn update(&mut self, mut i: i32, val: i32) {
        let v = val - self.nums[i as usize];
        self.nums[i as usize] = val;
        let mut i = (i + 1) as usize;
        while i < self.arr.len() {
            self.arr[i] += v;
            i += lsb(i as i32) as usize;
        }
    }
    fn sum(&self, i: i32) -> i32 {
        let mut i = (i + 1) as usize;
        let mut sum = 0;
        while i > 0 {
            sum += self.arr[i];
            i -= lsb(i as i32) as usize;
        }
        sum
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum(j) - self.sum(i - 1)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */
