impl Solution {
  pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
    nums.sort_unstable();
    let mut ret = f64::MAX;
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
      ret = ret.min((nums[right] as f64 + nums[left] as f64) / 2f64);
      left += 1;
      right -= 1;
    }
    ret
  }
}

impl Solution {
  pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let (mut t, mut b, mut l, mut right) = (rows, 0, cols, 0);
    for r in 0..rows {
      for c in 0..cols {
        if grid[r][c] == 0 { continue; }
        t = t.min(r);
        b = b.max(r);
        l = l.min(c);
        right = right.max(c);
      }
    }
    (b - t + 1) as i32 * (right - l + 1) as i32
  }
}

impl Solution {
  pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
    let mut plus = nums[0] as i64;
    let mut minus = nums[0] as i64;
    for i in 1..nums.len() {
      let n = nums[i];
      let plused = plus + n as i64;
      let minused = minus - n as i64;
      plus = plused.max(minused);
      minus = plused;
    }
    plus.max(minus)
  }
}

fn build_map(grid: &Vec<Vec<i32>>) -> Vec<Vec<Vec<Vec<i32>>>> {
  let rows = grid.len();
  let cols = grid[0].len();
  let mut top = vec![vec![vec![vec![rows; cols]; cols]; rows]; rows];
  let mut bottom = vec![vec![vec![vec![0; cols]; cols]; rows]; rows];
  let mut left = vec![vec![vec![vec![cols; cols]; cols]; rows]; rows];
  let mut right = vec![vec![vec![vec![0; cols]; cols]; rows]; rows];
  for t in 0..rows {
    for b in t..rows {
      for l in 0..cols {
        for r in l..cols {
          let is_one = grid[b][r] == 1;
          if r == 0 && b == 0 {
            if is_one {
              top[t][b][l][r] = 0;
              bottom[t][b][l][r] = 0;
              left[t][b][l][r] = 0;
              right[t][b][l][r] = 0;
            }
          } else if r == 0 {
            if is_one {
              top[t][b][l][r] = (top[t][b - 1][l][r]).min(b);
              bottom[t][b][l][r] = (bottom[t][b - 1][l][r]).max(b);
              left[t][b][l][r] = (left[t][b - 1][l][r]).min(r);
              right[t][b][l][r] = (right[t][b - 1][l][r]).max(r);
            } else {
              top[t][b][l][r] = (top[t][b - 1][l][r]);
              bottom[t][b][l][r] = (bottom[t][b - 1][l][r]);
              left[t][b][l][r] = (left[t][b - 1][l][r]);
              right[t][b][l][r] = (right[t][b - 1][l][r]);
            }
          } else if b == 0 {

          } else {
            top[t][b][l][r] = top[t][b][l][r - 1].min(top[t][b - 1][l][r]);
            bottom[t][b][l][r] = bottom[t][b][l][r - 1].max(bottom[t][b - 1][l][r]);
            left[t][b][l][r] = left[t][b][l][r - 1].min(left[t][b - 1][l][r]);
            right[t][b][l][r] = right[t][b][l][r - 1].max(right[t][b - 1][l][r]);
          }
        }
      }
    }
  }
}

impl Solution {
  pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {

  }
}
