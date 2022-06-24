use std::collections::BinaryHeap;
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        // must cast to i64 to avoid overflow
        let target: Vec<_> = target.into_iter().map(|b| b as i64).collect();
        let mut sum: i64 = target.iter().sum();
        let mut heap: BinaryHeap<_> = target.into_iter().collect();
        while let Some(max) = heap.pop() {
            if max == 1 {
                return true;
            }
            let remain = sum - max;
            if max <= remain || remain == 0 {
                return false;
            }
            let next = max % remain; // use max-remain will make max still the greatest
            let next = if next == 0 { remain } else { next }; // keep non zero remainer
            heap.push(next);
            sum = next + remain;
        }
        false
    }
}
