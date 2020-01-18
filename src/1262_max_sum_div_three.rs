impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let MAX = i32::max_value();
        let mut min_11 = MAX;
        let mut min_12 = MAX;
        let mut min_21 = MAX;
        let mut min_22 = MAX;
        let mut sum = 0;
        for n in nums {
            sum += n;
            if n % 3 == 1 {
                if n < min_11 {
                    min_12 = min_11;
                    min_11 = n;
                } else if n < min_12 {
                    min_12 = n;
                }
            } else if n % 3 == 2 {
                if n < min_21 {
                    min_22 = min_21;
                    min_21 = n;
                } else if n < min_22 {
                    min_22 = n;
                }
            }
        }
        if sum % 3 == 1 {
            if min_22 == MAX {
                sum - min_11
            } else {
                sum - min_11.min(min_21 + min_22)
            }
        } else if sum % 3 == 2 {
            if min_12 == MAX {
                sum - min_21
            } else {
                sum - min_21.min(min_11 + min_12)
            }
        } else {
            sum
        }
    }
}
