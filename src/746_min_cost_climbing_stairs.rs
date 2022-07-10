impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let mut m2 = 0;
        let mut m1 = 0;
        cost.push(0);
        for i in 2..cost.len() {
            let min = (m2 + cost[i - 2]).min(m1 + cost[i - 1]);
            m2 = m1;
            m1 = min;
        }
        m1
    }
}
