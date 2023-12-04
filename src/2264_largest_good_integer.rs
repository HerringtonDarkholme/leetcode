impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut ret = String::new();
        let chars: Vec<_> = num.chars().collect();
        for window in chars.windows(3) {
            if window[0] != window[1] || window[1] != window[2] {
                continue;
            }
            if ret.is_empty() {
                ret = window.iter().cloned().collect();
            } else if ret.chars().next().unwrap() < window[0] {
                ret = window.iter().cloned().collect();
            }
        }
        ret
    }
}
