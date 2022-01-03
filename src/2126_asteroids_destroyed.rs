impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        asteroids.sort();
        for a in asteroids {
            let a = a as i64;
            if mass >= a {
                mass += a;
            } else {
                return false;
            }
        }
        true
    }
}
