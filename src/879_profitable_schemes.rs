const M: i64 = 1_000_000_007;
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
      let mut ret = vec![vec![0; min_profit  as usize + 1]; n as usize + 1];
      ret[0][0] = 1;
      for (idx, g) in group.into_iter().enumerate() {
        let mut next = ret.clone();
        for i in 0..n {
          if i + g > n {
            break;
          }
          for j in 0..=min_profit {
            let new_profit = (j + profit[idx]).min(min_profit) as usize;
            let new_cost = (i + g) as usize;
            next[new_cost][new_profit] += ret[i as usize][j as usize];
            next[new_cost][new_profit] %= M;
          }
        }
        ret = next;
      }
      let mut sum = 0;
      for i in 0..=n {
        sum = (sum + ret[i as usize][min_profit as usize]) % M;
      }
      sum as i32
    }
}
