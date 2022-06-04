impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut ret = vec![];
        for c in s.chars() {
            if let Some((cc, mut prev)) = ret.pop() {
                if cc != c {
                    ret.push((cc, prev));
                    ret.push((c, 1));
                    continue;
                }
                prev += 1;
                if prev != k {
                    ret.push((c, prev));
                }
            } else {
                ret.push((c, 1));
            }
        }
        let mut s = String::new();
        for (c, cnt) in ret {
            for _ in 0..cnt {
                s.push(c);
            }
        }
        s
    }
}
//  [(a, cnt),]
