impl Solution {
    pub fn get_smallest_string(mut n: i32, mut k: i32) -> String {
        let mut ret = vec![];
        while n > 0 && k > n {
            let c = (k - n).min(25);
            ret.push((c as u8 + b'a') as char);
            k -= c + 1;
            n -= 1;
        }
        ret.extend(std::iter::repeat('a').take(n as usize));
        ret.into_iter().rev().collect()
    }
}
