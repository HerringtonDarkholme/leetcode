pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut r = vec![];
        for n in nums.into_iter().rev() {
            let index = match r.binary_search(&(n-1)) {
                Ok(i) => i+1,
                Err(i) => i,
            };
            ret.push(index as i32);
            r.insert(index, n);
        }
        ret.into_iter().rev().collect()
    }
}

fn get_sum(bit: &Vec<i32>, mut index: i32) -> i32 {
    let mut sum = 0;
    index += 1;
    while index > 0 {
        sum += bit[index as usize];
        index -= index & (-index);
    }
    sum
}

fn update(bit: &mut Vec<i32>, mut n: i32, mut index: i32, val: i32) {
    index += 1;
    while index <= n {
        bit[index as usize] += val;
        index += index & (-index);
    }
}

const B: i32 = 10001;
const SIZE: i32 = 2 * B + 1;
fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let mut bit = vec![0; SIZE as usize];
    for n in nums.into_iter().rev()  {
        let index = n + B;
        ret.push(get_sum(&bit, index - 1));
        update(&mut bit, SIZE, index, 1);
    }
    ret.reverse();
    ret
}

/*
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; nums.len()];
        merge_sort(nums.into_iter().enumerate().collect(), &mut count);
        count
    }
}

type Entry = (usize, i32);

fn merge_sort(mut nums: Vec<Entry>, count: &mut Vec<i32>) -> Vec<Entry> {
    if nums.len() <= 1 {
        return nums;
    }
    let mid = nums.len() / 2;
    let left = merge_sort(nums.drain(..mid).collect(), count);
    let right = merge_sort(nums, count);
    merge(left, right, count)
}

fn merge(nums1: Vec<Entry>, nums2: Vec<Entry>, count: &mut Vec<i32>) -> Vec<Entry> {
    let mut i = 0;
    let mut j = 0;
    let mut temp = vec![];
    let mut smaller = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i].1 > nums2[j].1 {
            temp.push(nums2[j]);
            smaller += 1;
            j += 1;
        } else {
            temp.push(nums1[i]);
            count[nums1[i].0] += smaller;
            i += 1;
        }
    }
    while i < nums1.len() {
        temp.push(nums1[i]);
        count[nums1[i].0] += smaller;
        i += 1;
    }
    while j < nums2.len() {
        temp.push(nums2[j]);
        j += 1;
    }
    temp
}
//        i
// num1  [2, 5]
// num2  [1, 6]
//        j
// temp  []
// smallerCount 0
// if num2[j] < num1[i]: smallerCount ++, write to temp
// else: count[index(nums1[i])] += smallerCount, write to temp
*/
