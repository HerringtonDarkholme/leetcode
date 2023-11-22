use std::collections::BTreeSet;
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut range = nums.len();
        for n in &nums {
            range = range.max(range + n.len());
        }
        let mut ret = vec![];
        let mut rows = BTreeSet::new();
        for i in 0..range {
            if i < nums.len() {
                rows.insert(i);
            }
            let mut to_delete = vec![];
            for &row in rows.iter().rev() {
                let col = i - row;
                let n = &nums[row];
                if col >= n.len() {
                    to_delete.push(row);
                } else {
                    ret.push(n[col]);
                }
            }
            for d in to_delete {
                rows.remove(&d);
            }
        }
        ret
    }
}
