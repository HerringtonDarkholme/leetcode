impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut last = nums[nums.len() - 1];
        let mut cost = 0;
        for num in nums.into_iter().rev().skip(1) {
            let (new, c) = add_num(last, num);
            last = new;
            cost += c as i64;
        }
        cost
    }
}

fn add_num(last: i32, new: i32) -> (i32, i32) {
    let cost = (new - 1) / last;
    (new / (cost + 1), cost)
}
// the question is equivalent to given the last element in the list
// what is the min split of adding a new number to make it sort
// and make the last number max
// we can prove that the min split is (new_num -1) / pred
// and the max last number is new_num/(min_split+1)
