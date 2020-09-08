impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut low = 0;
        let mut high = arr.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if k <= arr[mid] - (mid as i32) - 1 {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        let offset = if k <= arr[low] - (low as i32) - 1  { 
            0 
        } else {
            1 
        };
        (low as i32) + k + offset
    }
}
