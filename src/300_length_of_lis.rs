impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut ret: Vec<Vec<i32>> = vec![];
        for n in nums {
            let Err(i) = ret.binary_search_by_key(&n, |v| v[v.len() - 1]) else {
                continue;
            };
            let mut prev = if i == 0 { vec![] } else { ret[i - 1].clone() };
            prev.push(n);
            if i == ret.len() {
                ret.push(prev);
            } else {
                ret[i] = prev;
            }
        }
        ret.len() as i32
    }
}
// [2, 3, 7, 101]
// [2， 3， 4]
// [2, 3]
// [2]
