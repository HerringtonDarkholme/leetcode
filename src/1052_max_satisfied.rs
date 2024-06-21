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
/*

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut ret = 0;
        let mut unsat = Vec::with_capacity(customers.len());
        for i in 0..grumpy.len() {
            if grumpy[i] == 0 {
                ret += customers[i];
                unsat.push(0);
            } else {
                unsat.push(customers[i]);
            }
        }
        let mut max = 0;
        for i in 0..(minutes as usize) {
            max += unsat[i];
        }
        let mut curr = max;
        for i in (minutes as usize)..unsat.len() {
            curr -= unsat[i - minutes as usize];
            curr += unsat[i];
            max = max.max(curr);
        }
        ret + max
    }
}
*/
