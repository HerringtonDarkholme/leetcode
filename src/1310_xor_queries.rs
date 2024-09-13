impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pre = vec![0];
        let mut last = 0;
        for num in arr {
            last ^= num;
            pre.push(last);
        }
        queries.into_iter().map(|v| {
            pre[v[1] as usize + 1] ^ pre[v[0] as usize]
        }).collect()
    }
}
