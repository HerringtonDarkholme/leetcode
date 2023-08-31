impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut intervals = ranges
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if s == 0 { None } else { Some((0.max(i as i32 - s), i as i32 + s)) }
            })
            .collect::<Vec<_>>();
        intervals.sort_by_key(|(l, r)| (*l, -*r));
        let mut left = -1; // accepted interval reach
        let mut right = 0; // pending interval reach
        let mut ret = 0;
        for (l, r) in intervals {
            // impossible to cover
            if l > right {
                return -1;
            }
            // already found better interval
            if r <= right {
                continue;
            }
            // include current interval
            if l > left {
                left = right;
                right = r.max(right);
                ret += 1;
            } else if r > right {
                // update the most far-reaching interval
                right = r;
            }
            if right >= n {
                break;
            }
        }
        if right >= n { ret } else { -1 }
    }
}
