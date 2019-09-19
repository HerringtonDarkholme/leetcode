pub struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (start, dest) = if start > destination {
            (destination, start)
        } else {
            (start, destination)
        };
        let mut clock = 0;
        for i in start..dest {
            clock += distance[i as usize];
        }
        let mut counter = 0;
        for i in dest..(start + distance.len() as i32) {
            counter += distance[i as usize % distance.len()];
        }
        counter.min(clock)
    }
}
