impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut min = i32::MAX;
        for i in 1..arr.len() {
            min = min.min(arr[i] - arr[i - 1]);
        }
        let mut ret = vec![];
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min {
                ret.push(vec![arr[i - 1], arr[i]]);
            }
        }
        ret
    }
}
