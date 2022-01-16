impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut last = seats
            .iter()
            .enumerate()
            .find_map(|(i, &n)| if n == 1 { Some(i) } else { None })
            .unwrap();
        let mut max = last;
        for (i, &n) in seats.iter().enumerate().skip(last) {
            if n == 0 {
                continue;
            }
            max = max.max((i - last) / 2);
            last = i;
        }
        if seats[seats.len() - 1] == 0 {
            max = max.max(seats.len() - 1 - last);
        }
        max as i32
    }
}
