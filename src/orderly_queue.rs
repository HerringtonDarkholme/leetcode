pub struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut v: Vec<_> = s.chars().collect();
        if k > 1 {
            v.sort();
            return v.into_iter().collect()
        }
        let len = v.len();
        let mut max = 0;
        let mut i = 1;
        while i < len {
            if v[max] > v[i] {
                max = i;
            }
            if v[max] != v[i] {
                i += 1;
                continue;
            }
            for j in 1..len {
                let right = (i + j) % len;
                let left = (max + j) % len;
                if v[left] < v[right] {
                    break;
                }
                if v[left] > v[right] {
                    max = i;
                    break;
                }
            }
            i += 1;
        }
        v.rotate_left(max);
        v.into_iter().collect()
    }
}

#[test]
fn test() {
    for i in nested![
        ("abc", 1, "abc")
    ] {
        assert_eq!(Solution::orderly_queue(i.0.to_owned(), i.1), i.2.to_owned());
    }
}
