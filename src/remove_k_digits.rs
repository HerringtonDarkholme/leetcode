pub struct Solution;

impl Solution {
    pub fn remove_kdigits(nums: String, k: i32) -> String {
        let mut k = k as usize;
        let l = nums.len();
        let remain = l - k;
        let mut chars = Vec::with_capacity(l);
        for n in nums.chars() {
            let n = n.to_digit(10).unwrap();
            while k > 0 && !chars.is_empty() && *chars.last().unwrap() > n {
                chars.pop();
                k -= 1;
            }
            chars.push(n);
        }
        let ret = chars.into_iter()
            .take(remain)
            .skip_while(|p| *p == 0)
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("");
        if ret.is_empty() { "0".to_owned() } else { ret }
    }
}
