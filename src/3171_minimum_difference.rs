impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
      let mut i = 0;
      let mut cnt = 0;
      let mut bins = vec![0; 32];
      let mut min = i32::MAX;
      for j in 0..nums.len() {
        let mut n = add_to_bin(nums[j], &mut cnt, &mut bins);
        while i <= j {
          min = (n - k).abs().min(min);
          if n > k {
            break;
          }
          n = sub_from_bin(nums[i], &mut cnt, &mut bins);
          i += 1;
        }
      }
      min
    }
}

fn add_to_bin(mut n: i32, cnt: &mut i32, bins: &mut Vec<i32>) -> i32 {
  let mut i = 0;
  while n > 0 {
    bins[i] += n & 1;
    i += 1;
    n >>= 1;
  }
  *cnt += 1;
  bin_to_num(*cnt, bins)
}
fn sub_from_bin(mut n: i32, cnt: &mut i32, bins: &mut Vec<i32>) -> i32 {
  let mut i = 0;
  while n > 0 {
    bins[i] -= n & 1;
    i += 1;
    n >>= 1;
  }
  *cnt -= 1;
  bin_to_num(*cnt, bins)
}
fn bin_to_num(cnt: i32, bins: &[i32]) -> i32 {
  let mut ret = 0;
  if cnt == 0 {
    return ret;
  }
  for &b in bins.iter().rev() {
    ret *= 2;
    if b == cnt {
      ret += 1;
    }
  }
  ret
}
