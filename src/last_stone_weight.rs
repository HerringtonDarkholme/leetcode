// lc 1049
pub struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>() as usize;
        let mut current = vec![false; sum + 1];
        current[stones[0] as usize] = true;
        for i in 1..stones.len() {
            let stone = stones[i] as usize;
            let mut next = vec![false; sum + 1];
            for j in 0..sum {
                if current[j] {
                    next[j + stone] = true;
                    let abs = (j as i32 - stone as i32).abs() as usize;
                    next[abs] = true;
                }
            }
            current = next;
        }
        for i in 0..sum {
            if current[i] {
                return i as i32
            }
        }
        return -1
    }
}
