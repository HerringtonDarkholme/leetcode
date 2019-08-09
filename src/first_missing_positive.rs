struct Solution;

/*
 for an array of length l, the missing positive cannot be larger than len + 1
 so effectively we are finding the smallest missing number from 1 - len
 since we don't have extra space to record number, reusing vector is last resort
 rearrange nums to record the occurrence of all positive num < len
 example: [1, 2, 3, 4, 0, 6]
 the i-th number will record if i+1 occurs, e.g. if the first num == 1,
 it means 1 has occurred in nums.
 if i-th number is 0, it means i+1 doesn't exist
 we can rearrnage num by swapping: the number in swapped position is either 0 or index itself
 for i in 0..len, swap nums[i] with 0, to indicate no number seen,
 then the swapped old nums[i] should be the new swap target
 trick here is if num[i] == i+1, swap will terminate. and if we set occurrence as fixed number,
 the original number in vector wil confuse us. we need identification for number:
 number value itself is a good choice. value~position matches
*/
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let max = nums.len();
        // record all number in vector
        for i in 0..max {
            let mut j = nums[i] as usize; // swap out number occurred
            nums[i] = 0; // set number to zero to indicate non existing
            // if swapped out number lies in range 1-max
            while j > 0 && j <= max && nums[j - 1] != j as i32 {
                // and nums[j -1] != j means j-th number isn't swapped
                let temp = nums[j - 1] as usize;
                nums[j - 1] = j as i32;
                j = temp;
                // new j wil also be swapped, until number and position match
            }
        }
        // find the first missing number is to
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, n)| if n > 0 { None } else { Some(i) })
            .next()
            .or(Some(max))
            .map(|i| i as i32 + 1)
            .unwrap()
    }
}

#[test]
fn test() {
    for t in vec![
        (vec![1,2,0], 3),
        (vec![1,1,2,2,3,4,5,9], 6),
        (vec![3,4,-1,1], 2),
        (vec![7,8,9,11,12], 1),
        (vec![2,4,5,1,333,444], 3),
        (vec![94,-23,44,-33,122,33,0,0,0,0,0,0], 1),
        (vec![1, 0,0,0,0,0, 2, 0, 0, 0], 3),
        (vec![], 1),
    ] {
        assert_eq!(Solution::first_missing_positive(t.0), t.1);
    }
}
