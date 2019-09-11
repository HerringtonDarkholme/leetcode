pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        a.sort();
        let len = a.len();
        for i in 1..(len - 1) {
            if a[len - i] < a[len - i - 1] + a[len - i - 2] {
                return a[len - i] + a[len - i - 1] + a[len - i - 2]
            }
        }
        return 0
    }
}
