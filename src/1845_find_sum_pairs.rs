use std::collections::HashMap;
struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    count: HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut count = HashMap::new();
        for &i in nums2.iter() {
            *count.entry(i).or_insert(0) += 1;
        }
        FindSumPairs {
            nums1, nums2, count,
        }
    }
    
    fn add(&mut self, index: i32, val: i32) {
        let i = index as usize;
        let v = self.nums2[i];
        *self.count.get_mut(&v).unwrap() -= 1;
        self.nums2[i] += val;
        *self.count.entry(self.nums2[i]).or_insert(0) += 1;
    }
    
    fn count(&self, tot: i32) -> i32 {
        let mut r = 0;
        for &i in self.nums1.iter() {
            let remain = tot - i;
            r += *self.count.get(&remain).unwrap_or(&0);
        } 
        r
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
