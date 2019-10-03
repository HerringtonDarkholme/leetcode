pub struct Solution;

impl Solution {
    pub fn largest_time_from_digits(mut a: Vec<i32>) -> String {
        a.sort_by_key(|c| -c);
        for i in 0..4 {
            if a[i] > 2 {
                continue;
            }
            for j in 0..4 {
                if i == j || (a[i] == 2 && a[j] > 3) {
                    continue;
                }
                for k in 0..4 {
                    if k == i || k == j || a[k] > 5 {
                        continue;
                    }
                    for l in 0..4 {
                        if l == i || l == j || l == k {
                            continue
                        }
                        return format!("{}{}:{}{}", a[i], a[j], a[k], a[l])
                    }
                }
            }
        }
        "".to_string()
    }
}
