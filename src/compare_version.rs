// leetcode 165
pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1: Vec<i32> = version1
            .split(".")
            .map(|s| s.parse().unwrap())
            .collect();
        let version2: Vec<i32> = version2
            .split(".")
            .map(|s| s.parse().unwrap())
            .collect();
        for i in 0..version1.len().max(version2.len()) {
            let v1 = version1.get(i).unwrap_or(&0);
            let v2 = version2.get(i).unwrap_or(&0);
            if v1 > v2 {
                return 1
            } else if v1 < v2 {
                return -1
            }
        }
        0
    }
}
