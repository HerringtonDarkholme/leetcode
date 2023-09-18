impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut mat: Vec<_> = mat.into_iter().enumerate().map(|(i, v)| {
            (v.into_iter().filter(|n| *n == 1).count(), i)
        }).collect();
        mat.sort();
        mat[..k as usize].into_iter().map(|n| n.1 as i32).collect()
    }
}
