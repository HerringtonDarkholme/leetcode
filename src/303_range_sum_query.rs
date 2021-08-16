struct NumArray {
    sums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0];
        let mut s = 0;
        for n in nums {
            s += n;
            sums.push(s);
        }
        Self {
            sums,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let l = self.sums[left as usize];
        let r = self.sums[right as usize + 1];
        r - l
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
