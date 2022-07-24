struct Solution {
    areas: Vec<(i32, Vec<i32>)>,
    acc: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut areas = vec![];
        let mut acc = 0;
        for rect in rects {
            let area = (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            acc += area;
            areas.push((acc, rect));
        }
        Self {
            areas,
            acc,
        }
    }

    fn pick(&self) -> Vec<i32> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0, self.acc);
        let i = match self.areas.binary_search_by_key(&(num + 1), |a| a.0) {
            Ok(i) => i,
            Err(i) => i,
        };
        let point = &self.areas[i].1;
        let x = rng.gen_range(point[0], point[2] + 1);
        let y = rng.gen_range(point[1], point[3] + 1);
        vec![x, y]
    }
}
//[10,20]

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
