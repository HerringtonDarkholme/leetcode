impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        refill(plants, capacity_a, capacity_b)
    }
}
fn refill(plants: Vec<i32>, cap_a: i32, cap_b: i32) -> i32 {
    let mut r = 0;
    let mut cur_a = cap_a;
    let mut cur_b = cap_b;
    let mut i = 0;
    let mut j = plants.len() -1;
    while i < j {
        if cur_a < plants[i] {
            cur_a = cap_a;
            r += 1;
        }
        if cur_b < plants[j] {
            cur_b = cap_b;
            r += 1;
        }
        cur_a -= plants[i];
        cur_b -= plants[j];
        i += 1;
        j -= 1;
    }
    if i == j {
        if cur_a.max(cur_b) < plants[i] {
            r += 1;
        }
    }
    r
}
