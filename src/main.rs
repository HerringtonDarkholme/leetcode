#[macro_use]
mod util;
#[path = "./1309_freq_alphabets.rs"]
pub mod largest_divisible_subset;

fn main() {}

#[test]
fn test() {
    assert_eq!(Solution::minimize_result("247+38".into()), "2(47+38)");
    assert_eq!(Solution::minimize_result("12+34".into()), "1(2+3)4");
    assert_eq!(Solution::minimize_result("999+999".into()), "(999+999)");
}

struct Solution;

impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut even = vec![];
        let mut odd = vec![];
        let mut original = num;
        while num > 0 {
            let n = num % 10;
            if n % 2 == 0 {
                even.push(n);
            } else {
                odd.push(n);
            }
            num /= 10;
        }
        even.sort_by_key(|i| -i);
        odd.sort_by_key(|i| -i);
        let mut ret = vec![];
        while original > 0 {
            let n = original % 10;
            if n % 2 == 0 {
                ret.push(even.pop().unwrap());
            } else {
                ret.push(odd.pop().unwrap());
            }
            original /= 10;
        }
        ret.reverse();
        let mut r = 0;
        for n in ret {
            r = r * 10 + n;
        }
        r
    }
}

impl Solution {
    pub fn minimize_result(expression: String) -> String {
        minimize_result(expression)
    }
}
fn minimize_result(expression: String) -> String {
    let (left, right) = parse_num(&expression).unwrap();
    let lefts = break_num(left);
    let rights = break_num_right(right);
    let (l_paren, r_paren) = find_max(lefts, rights);
    assemble(expression, l_paren, r_paren)
}

fn parse_num(expression: &String) -> Option<(i32, i32)> {
    let mut expr = expression.split("+");
    let left: i32 = expr.next()?.parse().ok()?;
    let right: i32 = expr.next()?.parse().ok()?;
    Some((left, right))
}

fn break_num(num: i32) -> Vec<(i32, i32)> {
    let mut ret = vec![];
    let mut mutate = num;
    let mut right = 0;
    let mut i = 0;
    while mutate > 0 {
        right += mutate % 10 * 10i32.pow(i);
        i += 1;
        mutate /= 10;
        if num == right {
            ret.push((1, right));
        } else {
            ret.push(((num - right) / 10i32.pow(i), right));
        }
    }
    ret.reverse();
    ret
}

fn break_num_right(num: i32) -> Vec<(i32, i32)> {
    let mut ret = vec![];
    let mut mutate = num;
    let mut right = 0;
    let mut i = 0;
    while mutate > 0 {
        right += mutate % 10 * 10i32.pow(i);
        mutate /= 10;
        i += 1;
        if num == right {
            ret.insert(0, (right, 1));
        } else {
            ret.push(((num - right) / 10i32.pow(i), right));
        }
    }
    ret
}

fn find_max(lefts: Vec<(i32, i32)>, rights: Vec<(i32, i32)>) -> (usize, usize) {
    let mut min = i32::MAX;
    let mut min_i = (0, 0);
    for i in 0..lefts.len() {
        for j in 0..rights.len() {
            let n = lefts[i].0 * (lefts[i].1 + rights[j].0) * rights[j].1;
            if n < min {
                min_i = (i, j);
                min = n;
            }
        }
    }
    min_i
}

fn assemble(input: String, l_paren: usize, r_paren: usize) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    chars.insert(l_paren, '(');
    chars.insert(chars.len() - r_paren, ')');
    chars.into_iter().collect()
}

const M: i64 = 1_000_000_007;
use std::collections::BinaryHeap;
impl Solution {
    pub fn maximum_product(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut heap: BinaryHeap<_> = nums.into_iter().map(|k| -k).collect();
        for _ in 0..k {
            let p = heap.pop().unwrap();
            heap.push(p - 1);
        }
        let mut ret = 1i64;
        for n in heap {
            ret = (ret * -n as i64) % M;
        }
        ret as i32
    }
}
