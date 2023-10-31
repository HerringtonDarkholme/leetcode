impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(pref.len());
        let mut last = 0;
        for i in pref {
            ret.push(i ^ last);
            last = i;
        }
        ret
    }
}
