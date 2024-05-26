const M: i64 = 1_000_000_007;
impl Solution {
    pub fn maximum_sum_subsequence(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
      let mut ret = 0;
      let (mut take, mut no_take) = sub_seq(&nums);
      for query in queries {
        let pos = query[0] as usize;
        nums[pos] = query[1];
        ret += update(&mut take, &mut no_take, pos, &nums);
        ret %= M;
      }
      ret as i32
    }
}

fn sub_seq(nums: &[i32]) -> (Vec<i64>, Vec<i64>) {
  let mut take = vec![0; nums.len() + 1];
  let mut no_take = vec![0; nums.len() + 1];
  for i in 0..nums.len() {
    let num = nums[i] as i64;
    take[i + 1] = take[i].max(no_take[i] + num);
    no_take[i + 1] = take[i].max(no_take[i]);
  }
  (take, no_take)
}

fn update(take: &mut Vec<i64>, no_take: &mut Vec<i64>, pos: usize, nums: &[i32]) -> i64 {
  let len = nums.len();
  for i in pos..nums.len() {
    let num = nums[i] as i64;
    let new_take = take[i].max(no_take[i] + num);
    let new_no_take = take[i].max(no_take[i]);
    if new_take == take[i + 1] && new_no_take == no_take[i + 1] {
      break;
    }
    take[i + 1] = new_take;
    no_take[i + 1] = new_no_take;
  }
  take[len].max(no_take[len])
}
