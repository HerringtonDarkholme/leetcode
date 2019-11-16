// leetcode 1052
pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut total = 0;
        let x = x as usize;
        let mut y = x as usize;
        let mut happy = 0;
        let mut max = 0;

        for (i, (&c, &g)) in customers.iter().zip(grumpy.iter()).enumerate() {
            total += c * (1 - g);
            if y > 0 {
                y -= 1;
            } else {
                max = max.max(happy);
                happy -= customers[i - x] * grumpy[i - x];
            }
            happy += c * g;
        }
        total + max.max(happy)
    }
}
