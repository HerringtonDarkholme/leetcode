pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut prev = vec![];
        for (i, &h) in height.iter().enumerate() {
            while let Some(&p) = prev.last() {
                if height[p] > h {
                    break;
                }
                prev.pop();
                if prev.is_empty() {
                    break;
                }
                let last = *prev.last().unwrap();
                let h = std::cmp::min(h, height[last]);
                area += (h - height[p]) * (i - last - 1) as i32;
            }
            prev.push(i);
        }
        area
    }
}

#[test]
fn test() {
    let tests = vec!(
        (vec![0,1,0,2,1,0,1,3,2,1,2,1], 6),
        (vec![0,1,6,2,1,0,1,3,2,1,2,1], 9),
        (vec![0,0,0,1,2,0,3,5,0,0,8], 12),
        (vec![2,4,6,78,3,3,2,76,2,2,6,3,2,5,7,98,43,2,2,45,23], 835),
        (vec![2,0,0,0,2], 6),
    );
    for (test, num) in tests {
        assert_eq!(num, Solution::trap(test));
    }
}
