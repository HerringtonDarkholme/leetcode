struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = i32::MIN;
        for i in 0..matrix.len() {
            let mut row_sum = vec![0; matrix[0].len()];
            for row in i..matrix.len() {
                for col in 0..matrix[row].len() {
                    row_sum[col] += matrix[row][col];
                }
                update_result(&row_sum, k, &mut result);
                if result == k {
                    return result
                }
            }
        }
        return  result
    }
}

fn update_result(nums: &Vec<i32>, k: i32, result: &mut i32) {
    let mut sum = 0;
    let mut sorted_sum = std::collections::BTreeSet::new();
    sorted_sum.insert(0);
    for &n in nums {
        sum += n;
        if let Some(x) = sorted_sum.range(sum - k..).next() {
            *result = (*result).max(sum - x);
        }
        sorted_sum.insert(sum);
    }

}
