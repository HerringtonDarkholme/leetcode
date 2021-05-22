impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let nums = dissemble(num);
        let sorted = swap(nums);
        assemble(sorted)
    }
}

fn dissemble(mut num: i32) -> Vec<i32> {
    let mut v = vec![];
    while num != 0 {
        v.push(num % 10);
        num /= 10;
    }
    v
}

fn swap(mut nums: Vec<i32>) -> Vec<i32> {
    let mut max = -1;
    let mut j = -1;
    let mut l = -1;
    let mut r = 0;
    for i in 0..nums.len() {
        if nums[i] > max {
            max = nums[i];
            j = i as i32;
            continue;
        }
        if nums[i] < max {
            l = i as i32;
            r = j;
        }        
    }
    if l != -1 {
        nums.swap(r as usize, l as usize)
    }
    nums
}

fn assemble(nums: Vec<i32>) -> i32 {
    nums.into_iter().rev().fold(0, |acc, n| {
        acc * 10 + n
    })
}
