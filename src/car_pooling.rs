pub struct Solution;

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pass = [0; 1000];
        for t in trips {
            for i in t[1]..t[2] {
                let i = i as usize;
                pass[i] += t[0];
                if pass[i] > capacity {
                    return false
                }
            }
        }
        true
    }
}
