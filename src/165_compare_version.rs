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

/*
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = 0;
        let mut v2 = 0;
        let version1: Vec<_> = version1.bytes().collect();
        let version2: Vec<_> = version2.bytes().collect();
        let mut i = 0;
        let mut j = 0;
        while i < version1.len() || j < version2.len() {
            while i < version1.len() && version1[i] != b'.' {
                v1 = v1 * 10 + (version1[i] - b'0') as i32;
                i += 1;
            }
            i += 1;
            while j < version2.len() && version2[j] != b'.' {
                v2 = v2 * 10 + (version2[j] - b'0') as i32;
                j += 1;
            }
            j += 1;
            if v1 < v2 {
                return -1;
            } else if v1 > v2 {
                return 1;
            }
            v1 = 0;
            v2 = 0;
        }
        0
    }
}                             
                             
*/
