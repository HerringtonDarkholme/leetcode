impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut cache = vec![0; arr.len()];
        can_reach(&arr, start, &mut cache)
    }
}

fn can_reach(arr: &[i32], start: i32, cache: &mut [i32]) -> bool {
    if start as usize >= arr.len() || start < 0 {
        return false;
    }
    if cache[start as usize] != 0 {
        return cache[start as usize] == 1;
    }
    let delta = arr[start as usize];
    if delta == 0 {
        cache[start as usize] = 1;
        return true;
    }
    // set visited node as unreachable by default
    cache[start as usize] = -1;
    let can = can_reach(arr, start + delta, cache) || can_reach(arr, start - delta, cache);
    cache[start as usize] = if can { 1 } else { -1 };
    return can
}
