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

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut tree = vec![vec![]];
        for num in nums { build_tree(&mut tree, num); }
        build_set(&tree)
    }
}
fn build_tree(tree: &mut Vec<Vec<(i32, i32)>>, num: i32) {
    for i in (0..tree.len()).rev() {
        for (prev_idx, (prev, _)) in tree[i].iter().enumerate() {
            if num % prev != 0 { continue; }
            let entry = (num, prev_idx as i32);
            if i + 1 == tree.len() { tree.push(vec![]); }
            tree[i + 1].push(entry);
            return;
        }
    }
    tree[0].push((num, 0));
}

fn build_set(tree: &Vec<Vec<(i32, i32)>>) -> Vec<i32> {
    let (mut start, mut i) = (tree.len() - 1, 0 );
    let mut ret = vec![];
    loop {
        let (elem, next) = tree[start][i];
        ret.push(elem);
        if start == 0 { break ret; }
        start -= 1;
        i = next as usize;
    }
}
*/
