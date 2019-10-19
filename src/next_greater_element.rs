pub struct Solution;
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut v = num_to_vec(n);
        next_large(&mut v)
    }
}
fn next_large(v: &mut Vec<i32>) -> i32 {
    let mut last = v[0];
    for i in 1..v.len() {
        if v[i] < last {
            let mut j = 0;
            while j < i && v[j] <= v[i] {
                j += 1;
            }
            v.swap(i, j);
            reverse(v, 0, i - 1);
            return vec_to_num(v)
        }
        last = v[i]
    }
    -1
}
fn reverse(v: &mut Vec<i32>, mut start: usize, mut end: usize) {
    while start < end {
        v.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn num_to_vec(mut n: i32) -> Vec<i32> {
    let mut r = vec![];
    while n > 0 {
        r.push(n % 10);
        n /= 10;
    }
    r
}

fn vec_to_num(v: &Vec<i32>) -> i32 {
    let max = i32::max_value();
    let max_v = num_to_vec(max);
    let mut r = 0;
    if max_v.len() > v.len() {
        for &n in v.iter().rev() {
            r = r * 10 + n;
        }
    } else {
        assert_eq!(v.len(), max_v.len());
        let l = v.len();
        for i in 1..l {
            if v[l - i] > max_v[l - i] {
                return -1
            }
            r = r * 10 + v[l - i];
        }
    }
    r
}
