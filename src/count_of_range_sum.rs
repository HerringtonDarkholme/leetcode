pub struct Solution;
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;
        let mut v = vec![0];
        let mut count = 0;
        let mut sum = 0i64;
        for num in nums {
            sum += num as i64;
            let n = sum;
            let u = my_search(&v, n - lower + 1);
            let l = my_search(&v, n - upper);
            count += u - l;
            let i = v.binary_search(&n).unwrap_or_else(|x| x);
            v.insert(i, n);
        }
        count as i32
    }
}
fn my_search(v: &Vec<i64>, target: i64) -> usize {
    let mut low = 0;
    let mut high = v.len() - 1;
    while low < high {
        let mid = low + (high - low) / 2;
        if v[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if v[low] >= target {
        low
    } else {
        low + 1
    }
}
