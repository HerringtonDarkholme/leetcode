pub struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut last = 0;
        flowerbed.insert(0, 0);
        flowerbed.push(0);
        if n == 0 {
            return true
        }
        for i in 1..flowerbed.len()-1 {
            if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 && flowerbed[i] == 0 {
                flowerbed[i] = 1;
                n -= 1;
                if n == 0 {
                    return true
                }
            }
        }
        false
    }
}
