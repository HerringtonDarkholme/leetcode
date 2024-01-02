impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut occurrences = vec![0; 201];
        let mut ret = vec![];
        for n in nums {
            let occ = occurrences[n as usize];
            if occ >= ret.len() {
                ret.push(vec![n]);
            } else {
                ret[occ].push(n);
            }
            occurrences[n as usize] += 1;
        }
        ret
    }
}
