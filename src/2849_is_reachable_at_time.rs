impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if fx == sx && fy == sy && t == 1 { return false }
        (fx - sx).abs() <= t && (fy - sy).abs() <= t
    }
}
