pub struct Solution;
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut low = 0;
        let k = k as usize;
        let mut high = arr.len() - k;
        while low < high {
            let mid = low + (high - low) / 2;
            let too_small = x - arr[mid] > arr[mid + k] -x;
            if too_small {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        arr[low..low+k].to_vec()
    }
}
