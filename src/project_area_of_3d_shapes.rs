pub struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut xy_area = 0;
        let mut xz_area = 0;
        let mut yz_areas = [0; 50];
        for (x, ys) in grid.iter().enumerate() {
            let mut max_z_on_x = 0;
            for (y, &z) in ys.iter().enumerate() {
                if z > 0 {
                    xy_area += 1;
                }
                if z > max_z_on_x {
                    max_z_on_x = z;
                }
                if yz_areas[y] < z {
                    yz_areas[y] = z;
                }
            }
            xz_area += max_z_on_x;
        }
        xy_area + xz_area + yz_areas.iter().sum::<i32>()
    }
}
