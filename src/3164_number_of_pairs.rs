use std::collections::HashMap;
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
      let mut ret = 0;
      let nums1: Vec<_> = nums1.into_iter().filter_map(|n| if n % k == 0 { Some(n / k) } else { None }).collect();
      let Some(&max) = nums1.iter().max() else { return 0; };
      let nums1 = counter(nums1);
      let nums2 = counter(nums2);
      for (num, count) in nums2 {
        let mut key = num;
        while key <= max {
          if let Some(c2) = nums1.get(&key) {
            ret += c2 * count;
          }
          key += num;
        }
      }
      ret as i32
    }
}

fn counter(nums: Vec<i32>) -> HashMap<i32, usize> {
  let mut ret = HashMap::new();
  for num in nums {
    *ret.entry(num).or_insert(0) += 1;
  }
  ret
}

