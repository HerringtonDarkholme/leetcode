impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut i = 0;
        let mut ret = vec![];
        for j in 1..=n {
            ret.push("Push".into());
            if j == target[i] {
                i += 1;
            } else if j < target[i] {
                ret.push("Pop".into());
            }
            if i >= target.len() {
                break;
            }
        }
        ret
    }
}
