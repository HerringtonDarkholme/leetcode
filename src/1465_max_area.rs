impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal = horizontal_cuts;
        let mut vertical = vertical_cuts;
        horizontal.insert(0, 0);
        horizontal.push(h);
        vertical.insert(0, 0);
        vertical.push(w);
        horizontal.sort();
        vertical.sort();
        let max_w = horizontal.windows(2).map(|v| v[1] - v[0]).max().unwrap() as i64;
        let max_h = vertical.windows(2).map(|v| v[1] - v[0]).max().unwrap() as i64;
        (max_w * max_h % 1_000_000_007) as i32
    }
}
