impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut curr = 0i64; 
        // given an array can form 1~n by sum
        // adding a num to the array can make it form 1~n+num
        let mut added = 0;
        for num in nums {
            while num as i64 > curr + 1 && curr < n as i64 {
                curr += curr + 1;
                added += 1;
            }
            curr += num as i64;
            if curr >= n as i64 {
                break;
            }
        }
        while curr < n as i64 {
            curr += curr + 1;
            added += 1;
        }
        added
    }
}
// 1 1 2 3 8 9 
