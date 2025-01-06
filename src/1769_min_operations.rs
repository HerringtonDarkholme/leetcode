impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut right_cost = vec![0; boxes.len()];
        // n => num of 1, d => total cost to move 1
        let (mut n, mut d) = (0, 0); 
        for (i, c) in boxes.bytes().enumerate().rev() {
            right_cost[i] = d;
            n += if c == b'1' { 1 } else { 0 };
            d += n;
        }
        let mut ret = Vec::with_capacity(boxes.len());
        let (mut n, mut d) = (0, 0);
        for (i, c) in boxes.chars().enumerate() {
            ret.push(d + right_cost[i]);
            n += if c == '1' { 1 } else { 0 };
            d += n;
        }
        ret
    }
}
// n = 3, d = 11
// 001011
// +85310 move left
// 000124 move right
