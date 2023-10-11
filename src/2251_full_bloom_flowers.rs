impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut starts: Vec<_> = flowers.iter().map(|v| v[0]).collect();
        let mut ends: Vec<_> = flowers.iter().map(|v| v[1]).collect();
        starts.sort();
        ends.sort();
        let mut ret = vec![];
        for p in people {
            let started = first_greater(&starts, p);
            let ended = first_greater(&ends, p - 1);
            ret.push((started - ended) as i32);
        }
        ret
    }
}
fn first_greater(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if nums[left] <= target {
        left + 1
    } else {
        left
    }
}
