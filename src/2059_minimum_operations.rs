use std::collections::{VecDeque};
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut heap = VecDeque::new();
        heap.push_back((0, start));
        let mut visited = vec![false; 1001];
        visited[start as usize] = true;
        while let Some((step, num)) = heap.pop_front() {
            for next in nums.iter().flat_map(|&n| [num + n, num - n, num ^ n]) {
                if next == goal {
                    return step + 1;
                }
                if !(0..=1000).contains(&next) || visited[next as usize] {
                    continue;
                }
                visited[next as usize] = true;
                heap.push_back((step + 1, next));
            }
        }
        -1
    }
}
