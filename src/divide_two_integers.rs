pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let sign = if dividend > 0 {
            if divisor > 0 { 1 } else { -1 }
        } else {
            if divisor > 0 { -1 } else { 1 }
        };
        if dividend == i32::min_value() && divisor == -1 {
            return i32::max_value()
        }
        let mut dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };
        let mut ret = 0;
        loop {
            if dividend > divisor {
                break ret * sign
            }
            let mut temp = divisor;
            let mut quotidian = 1;
            while dividend - temp < temp {
                temp <<= 1;
                quotidian <<= 1;
            }
            dividend -= temp;
            ret += quotidian;
        }
    }
}
