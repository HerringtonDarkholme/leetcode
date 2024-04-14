impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
      let coins = preprocess(coins);
      let k = k as i64;
      let mut left = 1;
      let mut right = coins[0] * k + 1;
      let combination = combine_coins(&coins);
      while left < right {
        let mid = left + (right - left) / 2;
        let pos = find_pos(mid, &combination);
        if pos >= k {
          right = mid;
        } else {
          left = mid + 1;
        }
      }
      find_nearest(left, &coins)
    }
}

fn find_nearest(mut i: i64, coins: &[i64]) -> i64 {
  loop {
    for &coin in coins {
      if i % coin == 0 {
        return i
      }
    }
    i += 1;
  }
}

fn combine_coins(coins: &[i64]) -> Vec<(i64, i64)> {
  let mut intersections = vec![(1, 0)];
  for &coin in coins {
    let mut next = intersections.clone();
    for (l, cnt) in intersections {
      let next_l = lcm(l, coin);
      next.push((next_l, cnt + 1));
    }
    intersections = next;
  }
  intersections.remove(0); // remove (1, 0)
  intersections.sort();
  intersections
}

fn find_pos(i: i64, combination: &[(i64, i64)]) -> i64 {
  let mut pos = 0;
  // lcm, element cnt
  for (l, cnt) in combination {
    if *l > i { break; }
    pos += i / l * if cnt % 2 == 1 { 1 } else { -1 };
  }
  pos
}

fn preprocess(mut coins: Vec<i32>) -> Vec<i64> {
  coins.sort();
  let mut ret = vec![];
  for coin in coins {
    // remove duplicate multiple
    if ret.iter().any(|&r| coin % r as i32 == 0) {
      continue;
    }
    ret.push(coin as i64);
  }
  ret
}


fn gcd(a: i64, b: i64) -> i64 {
  let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
  while b > 0 {
    (a, b) = (b, a % b);
  }
  a
}
fn lcm(a: i64, b: i64) -> i64 {
  a * b / gcd(a, b)
}
