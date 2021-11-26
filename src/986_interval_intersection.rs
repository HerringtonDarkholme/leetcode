pub struct Solution;

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < a.len() && j < b.len() {
            let ia = &a[i];
            let ib = &b[j];
            if ia[0] > ib[1] {
                j += 1;
                continue;
            }
            if ia[1] < ib[0] {
                i += 1;
                continue;
            }
            ret.push(vec![
                std::cmp::max(ia[0], ib[0]),
                std::cmp::min(ia[1], ib[1]),
            ]);
            if ia[1] < ib[1] {
                i += 1;
            } else {
                j += 1;
            }
        }
        ret
    }
}

#[test]
fn test() {
    let a = vec![vec![0,2],vec![5,10],vec![13,23],vec![24,25]];
    let b = vec![vec![1,5],vec![8,12],vec![15,24],vec![25,26]];
    let expect = vec![vec![1,2], vec![5, 5], vec![8, 10], vec![15, 23], vec![24, 24], vec![25, 25]];
    assert_eq!(Solution::interval_intersection(a, b), expect);
}

/*
impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v = vec![];
    helper(&first_list, &second_list, &mut v);
    v
    }
}

fn helper<'a>(mut l1: &'a[Vec<i32>], mut l2: &'a[Vec<i32>], dest: &'a mut Vec<Vec<i32>>) {
    if l1.is_empty() || l2.is_empty() {
        return;
    }
    let mut a = &l1[0];
    let mut b = &l2[0];
    if a[0] > b[0] {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut l1, &mut l2);
    }
    if a[1] < b[0] {
        return helper(&l1[1..], l2, dest);
    }
    if a[1] >= b[0] && a[1] <= b[1] {
        dest.push(vec![b[0], a[1]]);
        return helper(&l1[1..], l2, dest);
    }
    dest.push(vec![b[0], b[1]]);
    helper(l1, &l2[1..], dest);
}
*/
