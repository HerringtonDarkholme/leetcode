impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let chars: Vec<_> = s.chars().map(|c| (c as i32- '0' as i32)).collect();
        let z = chars.iter().filter(|&&c| c == 0).count() as i32;
        let o = chars.iter().filter(|&&c| c == 1).count() as i32;
        if (z - o).abs() > 1 {
            return -1
        }
        if z > o {
            swaps(chars, 0)
        } else if o > z {
            swaps(chars, 1)
        } else {
            let zc = swaps(chars.clone(), 0);
            let oc = swaps(chars, 1);
            zc.min(oc)
        }
    }
}
fn swaps(mut v: Vec<i32>, mut leading: i32) -> i32 {
    let mut s = 0;
    let mut r = 0;
    for i in 0..v.len() {
        if v[i] == leading {
            leading = 1 - leading;
            continue;
        }
        s += 1;
        r = r.max(i + 1);
        for j in r..v.len() {
            if v[j] == leading && j % 2 != i % 2 {
                v.swap(i, j);
                r = j;
                break;
            }
        }
        leading = 1 - leading;
    }
    s
}
