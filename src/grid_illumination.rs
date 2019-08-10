pub struct Solution;


#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point(i32, i32);

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut x_light = HashMap::new();
        let mut y_light = HashMap::new();
        // diagnoal line has slope of 1 or -1
        // (y1 - y2 == -(x1 - x2) =>> y1 + x1 == y2 + x2
        let mut diag_light_sum = HashMap::new(); // every pos on same diag has same x+y sum
        // (y1 - y2) == (x1 - x2) =>> y1 - x1 = y2 - x2
        let mut diag_light_diff = HashMap::new(); // every pos on same diag has same x-y diff
        let mut lights = HashSet::with_capacity(lamps.len());
        for l in lamps {
            let (x, y) = (l[0], l[1]);
            *x_light.entry(x).or_insert(0) += 1;
            *y_light.entry(y).or_insert(0) += 1;
            *diag_light_sum.entry(x + y).or_insert(0) += 1;
            *diag_light_diff.entry(x - y).or_insert(0) += 1;
            lights.insert(Point(x, y));
        }
        let mut ret = vec![];
        for q in queries {
            let (x, y) = (q[0], q[1]);
            if *x_light.get(&x).unwrap_or(&0) > 0 ||
               *y_light.get(&y).unwrap_or(&0) > 0 ||
               *diag_light_sum.get(&(x + y)).unwrap_or(&0) > 0 ||
               *diag_light_diff.get(&(x - y)).unwrap_or(&0) > 0 {
                ret.push(1);
            } else {
                ret.push(0);
            }
            for i in -1..=1 {
                for j in -1..=1 {
                    let p = Point(x+i, y+j);
                    if lights.get(&p).is_none() {
                        continue;
                    }
                    lights.remove(&p);
                    *x_light.get_mut(&p.0).unwrap() -= 1;
                    *y_light.get_mut(&p.1).unwrap() -= 1;
                    *diag_light_sum.get_mut(&(p.0 + p.1)).unwrap() -= 1;
                    *diag_light_diff.get_mut(&(p.0 - p.1)).unwrap() -= 1;

                }
            }
        }
        ret
    }
}

#[test]
fn test() {
    Solution::grid_illumination(5, nested![[0,0],[0,4]], nested![[0,4],[0,1],[1,4]]);
}
