use std::collections::HashMap;
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut map = HashMap::new();
        let mut sum = 0;
        for bean in beans {
            *map.entry(bean as i64).or_insert(0) += 1;
            sum += bean as i64;
        }
        let mut stack: Vec<_> = map.into_iter().collect();
        stack.sort_by_key(|v| std::cmp::Reverse(*v));
        let mut min = i64::MAX;
        let mut removed = 0;
        let mut count = 0;
        let mut last = 0;
        for (bean, cnt) in stack {
            sum -= bean * cnt;
            removed += (last - bean) * count;
            count += cnt;
            last = bean;
            min = min.min(sum + removed);
        }
        min
    }
}
