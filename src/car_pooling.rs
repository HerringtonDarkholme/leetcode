pub struct Solution;

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut locs = vec![];
        for trip in trips {
            locs.push((trip[1], trip[0]));
            locs.push((trip[2], -trip[0]));
        }
        locs.sort();
        let mut cur = 0;
        for loc in locs {
            cur += loc.1;
            if cur > capacity {
                return false
            }
        }
        true
    }
}
