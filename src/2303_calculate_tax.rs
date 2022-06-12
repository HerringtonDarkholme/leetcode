impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut ret = 0.0;
        let mut last = 0;
        for bracket in brackets {
            ret += ((bracket[0].min(income) - last) * bracket[1]) as f64 / 100.0;
            if income < bracket[0] {
                break;
            }
            last = bracket[0];
        }
        ret
    }
}

