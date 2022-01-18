impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut m = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] != 0 {
                continue;
            }
            let prev_empty = i == 0 || flowerbed[i - 1] == 0;
            let next_empty = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;
            if prev_empty && next_empty {
                m += 1;
                flowerbed[i] = 1;
            }
        }
        m >= n
    }
}
