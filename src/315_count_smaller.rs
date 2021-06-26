pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut r = vec![];
        for n in nums.into_iter().rev() {
            let index = match r.binary_search(&(n-1)) {
                Ok(i) => i+1,
                Err(i) => i,
            };
            ret.push(index as i32);
            r.insert(index, n);
        }
        ret.into_iter().rev().collect()
    }
}

fn get_sum(bit: &Vec<i32>, mut index: i32) -> i32 {
    let mut sum = 0;
    index += 1;
    while index > 0 {
        sum += bit[index as usize];
        index -= index & (-index);
    }
    sum
}

fn update(bit: &mut Vec<i32>, mut n: i32, mut index: i32, val: i32) {
    index += 1;
    while index <= n {
        bit[index as usize] += val;
        index += index & (-index);
    }
}

const B: i32 = 10001;
const SIZE: i32 = 2 * B + 1;
fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let mut bit = vec![0; SIZE as usize];
    for n in nums.into_iter().rev()  {
        let index = n + B;
        ret.push(get_sum(&bit, index));
        update(&mut bit, SIZE, index, 1);
    }
    ret.reverse();
    ret
}
