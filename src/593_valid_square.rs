impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let p12 = len(&p1, &p2);
        let p13 = len(&p1, &p3);
        let p14 = len(&p1, &p4);
        if p12 == 0 || p13 == 0 || p14 == 0 {
            return false
        }
        if p12 == p13 {
            if is_perpendicular(&p1, &p2, &p3) {
                len(&p4, &p2) == p12 && len(&p4, &p3) == p12
            } else {
                false
            }
        } else if p12 == p14 {
            if is_perpendicular(&p1, &p2, &p4) {
                len(&p3, &p2) == p12 && len(&p3, &p4) == p12
            } else {
                false
            }
        } else if p13 == p14 {
            if is_perpendicular(&p1, &p3, &p4) {
                len(&p2, &p3) == p13 && len(&p2, &p4) == p13
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn len(p1: &[i32], p2: &[i32]) -> i32 {
    (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
}

fn is_perpendicular(p1: &[i32], p2: &[i32], p3: &[i32]) -> bool {
    let p12x = p1[0] - p2[0];
    let p13x = p1[0] - p3[0];
    let p12y = p1[1] - p2[1];
    let p13y = p1[1] - p3[1];
    (p12x * p13x + p12y * p13y) == 0
}
