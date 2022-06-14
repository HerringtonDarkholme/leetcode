impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut last = vec![0; triangle.len() + 1];
        for origin in triangle.into_iter().rev() {
            last = reduce(last);
            for i in 0..origin.len() {
                last[i] += origin[i];
            }
        }
        last[0]
    }
}
fn reduce(mut v: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for i in 1..v.len() {
        ret.push(v[i].min(v[i - 1]));
    }
    ret
}

// starting from the last row, compute the smaller one of two adjacent row
// min path sum must exists in the reduded smaller row, add it with prev row
// repeat the process until the root will yield the min path sum
