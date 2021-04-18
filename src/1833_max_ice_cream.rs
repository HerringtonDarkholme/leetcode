use std::collections::BinaryHeap;
struct Solution;
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for c in costs {
            if sum + c <= coins {
                sum += c;
                heap.push(c);
                continue;
            }
            if let Some(&n) = heap.peek() {
                if n > c {
                    heap.pop();
                    sum -= n;
                    heap.push(c);
                    sum += c;
                }
            }

        }
        heap.len() as i32
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use proptest::collection::vec;
    use super::Solution;
    proptest! {
        #[test]
        fn test(
            costs in vec(1..1_000, 1..1_000),
            coins in 1..1_000,
        ) {
            let mut costs = costs;
            let max = Solution::max_ice_cream(costs.clone(), coins) as usize;
            costs.sort();
            let mut sum = 0;
            for i in 0..max {
                sum += costs[i];
            }
            assert!(sum <= coins);
            if costs.len() > max {
                assert!(sum + costs[max + 1] > coins);
            }
        }
    }

}
