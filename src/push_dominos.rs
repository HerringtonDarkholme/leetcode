pub struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut ds = dominoes.chars().collect::<Vec<_>>();
        aux(&mut ds);
        ds.iter().collect()
    }
}

fn aux(ds: &mut [char]) {
    let mut s = 0;
    let mut e = ds.len() - 1;
    while s <= e {
        let start = s;
        let end = e;
        while s <= e && ds[s] == '.' {
            s += 1;
        }
        if s > e {
            break;
        }
        if ds[s] == 'L' {
            for i in start..s {
                ds[i] = 'L';
            }
            s += 1;
        } else {
            let mut last_r = s;
            while s <= e && ds[s] != 'L' {
                if ds[s] == 'R' {
                    for i in last_r..s {
                        ds[i] = 'R';
                    }
                    last_r = s;
                }
                s += 1;

            }
            if s > e {
                for i in last_r..=e {
                    ds[i] = 'R';
                }
                break;
            }
            if ds[s] == 'L' {
                let mut i = last_r + 1;
                let mut j = s - 1;
                while i < j {
                    ds[i] = 'R';
                    ds[j] = 'L';
                    i += 1;
                    j -= 1;
                }
            } else {
                for i in start..s {
                    ds[i] = 'R';
                }
            }
            s += 1;
        }

        while e >= s && ds[e] == '.' {
            e -= 1;
        }

        if s > e {
            break;
        }

        if ds[e] == 'R' {
            for i in e..=end {
                ds[i] = 'R';
            }
            e -= 1;
        } else {
            let mut last_l = e;
            while s <= e && ds[e] != 'R' {
                if ds[e] == 'L' {
                    for i in e..last_l {
                        ds[i] = 'L';
                    }
                    last_l = e;
                }
                e -= 1;

            }
            if s > e {
                for i in e..last_l {
                    ds[i] = 'L';
                }
                break;
            }
            if ds[e] == 'R' {
                let mut i = e + 1;
                let mut j = last_l - 1;
                while i < j {
                    ds[i] = 'R';
                    ds[j] = 'L';
                    i += 1;
                    j -= 1;
                }
            } else {
                for i in e..end {
                    ds[i] = 'L';
                }
            }
            e -= 1;
        }
    }

}
