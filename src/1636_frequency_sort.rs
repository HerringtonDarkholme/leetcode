impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut count = std::collections::HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }
        let mut arr = count.into_iter().collect::<Vec<_>>();
        arr.sort_by_key(|c| (c.1, -c.0));
        let mut ret = vec![];
        for (n, cnt) in arr {
            ret.extend(std::iter::repeat(n).take(cnt));
        }
        ret
    }
}
