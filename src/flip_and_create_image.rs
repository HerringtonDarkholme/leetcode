// lc 832
pub struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for r in a.iter_mut() {
            flip(r);
            invert(r);
        }
        a
    }
}

fn flip(a: &mut Vec<i32>) {
    let mut i = 0;
    let mut j = a.len() - 1;
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn invert(a: &mut Vec<i32>) {
    for i in 0..a.len() {
        a[i] = 1 - a[i];
    }
}
