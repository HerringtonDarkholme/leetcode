impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        // use nums itself to track if number exist
        while i < nums.len() {
            if nums[i] == i as i32 + 1 || nums[i] <= 0 {
                i += 1;
                continue; // already sorted or need ignore
            }
            if nums[i] > nums.len() as i32 {
                nums[i] *= -1; // oob
                i += 1;
                continue;
            }
            let target = nums[i] as usize - 1;
            nums.swap(i, target); // note, it will swap at most n times
            // because a position will be either ignorable or swapped 
            // or direct to a new unswapped position. a swapped position 
            // will be marked as done in next line. ignored/swapped will be marked.
            if target + 1 == nums[i] as usize {
                i += 1;
            }
        }
        for i in 0..nums.len() {
            if i + 1 != nums[i] as usize {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}
