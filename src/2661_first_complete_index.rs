impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let row_count = mat.len();
        let col_count = mat[0].len();
        let mut row_painted = vec![0; row_count];
        let mut col_painted = vec![0; col_count];
        let (rows, cols) = gen_map(&mat);
        for (i, num) in arr.into_iter().enumerate() {
            let (r, c) = (rows[num as usize], cols[num as usize]); // 1 -> 0, 0
            row_painted[r] += 1;
            col_painted[c] += 1;
            if row_painted[r] == col_count || col_painted[c] == row_count {
                return i as i32
            }
        }
        -1 // impossible
    }
}

fn gen_map(mat: &Vec<Vec<i32>>) -> (Vec<usize>, Vec<usize>) {
    let size = mat.len() * mat[0].len();
    let mut rows = vec![0; size + 1];
    let mut cols = vec![0; size + 1];
    for (r, nums) in mat.iter().enumerate() {
        for (c, &num) in nums.iter().enumerate() {
            rows[num as usize] = r;
            cols[num as usize] = c;
        }
    }
    (rows, cols)
}

