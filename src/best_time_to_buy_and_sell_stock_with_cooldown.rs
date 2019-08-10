pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cache = vec![-1; prices.len()];
        Solution::aux(&prices, 0, &mut cache)
    }
    fn aux(prices: &[i32], start: usize, cache: &mut [i32]) -> i32 {
        if start >= cache.len() {
            return 0
        }
        if cache[start] >= 0 {
            return cache[start]
        }
        let mut r = 0;
        for i in (start + 1)..prices.len() {
            if prices[start] > prices[i] {
                r = r.max(Solution::aux(prices, i, cache));
            } else {
                let tmp = prices[i] - prices[start];
                r = r.max(tmp + Solution::aux(prices, i + 2, cache));
            }
        }
        cache[start] = r;
        r
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (vec![1, 2, 3, 0, 2], 3),
        (vec![1, 2, 3, 0, 5], 6),
        (vec![1, 1, 1, 1], 0),
        (vec![4, 3, 2, 1], 0),
        (vec![], 0),
    ], Solution::max_profit);
}
