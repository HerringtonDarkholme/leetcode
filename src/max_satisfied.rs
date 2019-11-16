pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut total = 0;
        let mut i = 0;
        let mut y = x as usize;
        let mut happy = 0;
        while y > 0 {
            total += customers[i] * (1 - grumpy[i]);
            happy += customers[i] * grumpy[i];
            i += 1;
            y -= 1;
        }
        let mut max = happy;
        while i < customers.len() {
            total += customers[i] * (1 - grumpy[i]);
            happy -= customers[i - x as usize] * grumpy[i - x as usize];
            happy += customers[i] * grumpy[i];
            max = max.max(happy);
            i += 1;
        }
        total + max
    }
}
