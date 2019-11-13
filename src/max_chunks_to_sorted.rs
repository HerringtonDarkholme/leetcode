// leetcode 768
pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(mut arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut largest = arr[0];
        let mut smallest = vec![i32::min_value(); len];
        let mut i = 0;
        smallest[len - 1] = arr[len - 1];
        for i in (0..len-1).rev() {
            smallest[i] = smallest[i+1].min(arr[i]);
        }
        smallest.push(i32::max_value());
        arr.push(i32::min_value());
        let mut ret = 0;
        for i in 0..len {
            largest = arr[i].max(largest);
            let small = smallest[i + 1];
            if largest <= small {
                ret += 1;
                largest = arr[i+1];
            }
        }
        ret
    }
}
