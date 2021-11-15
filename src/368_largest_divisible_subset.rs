struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        if nums.is_empty() {
            return vec![];
        }
        let mut rets = vec![
            vec![vec![nums.remove(0)]]
        ];
        for num in nums {
            'a: for (i, lists) in rets.iter_mut().enumerate().rev() {
                for list in lists.iter_mut() {
                    let last = list[i];
                    if num % last == 0 {
                        let mut new_list = list.clone();
                        new_list.push(num);
                        if i + 1 >= rets.len() {
                            rets.push(vec![new_list]);
                        } else {
                            rets[i + 1].push(new_list)
                        }
                        break 'a;
                    }
                }
            }
            rets[0].push(vec![num])
        }
        rets.pop().unwrap().remove(0)
    }
}

/*
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut rets: Vec<Vec<i32>> = vec![];
        for i in nums {
            let mut r = None;
            for v in rets.iter() {
                if i % v.last().unwrap() == 0 {
                    let mut temp = v.clone();
                    temp.push(i);
                    r = Some(temp);
                    break;
                }
            }
            rets.push(r.unwrap_or(vec![i]));
            rets.sort_by(|v1, v2| v2.len().cmp(&v1.len()));
        }
        rets.into_iter().next().unwrap_or(vec![])
    }
}
*/
