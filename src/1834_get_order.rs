use std::collections::BTreeMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        if tasks.is_empty() {
            return vec![]
        }
        let mut times = BTreeMap::new();
        for (i, task) in tasks.iter().enumerate() {
            times.
                entry(task[0]).
                or_insert_with(|| vec![]).
                push(i);
        }
        let mut available: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        let mut ret = vec![];
        let mut last = 0;
        for (now, idx) in times {
            while now > last && !available.is_empty() {
                let (duration, i) = available.pop().unwrap().0;
                last = last.max(tasks[i][0]) + duration;
                ret.push(i as i32);
            }
            available.extend(idx.iter().map(|&i| {
                Reverse((tasks[i][1], i))
            }));
            println!("now: {}, last {}, availabel{:?}, ret{:?}", now, last, available, ret);
            if now < last {
                continue;
            }
            let (duration, i) = available.pop().unwrap().0;
            last = last.max(tasks[i][0]) + duration;
            ret.push(i as i32);
        }
        let mut v = available.into_sorted_vec();
        v.reverse();
        ret.extend(v.iter().map(|x| (x.0).1 as i32));
        ret
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::Solution;
        let cases = nested![
            [1,2],[2,4],[3,2],[4,1]
        ];
        assert_eq!(Solution::get_order(cases), vec![0,2,3,1]);
        let cases = nested![
            [19,13],[16,9],[21,10],[32,25],[37,4],[49,24],[2,15],[38,41],[37,34],[33,6], [45,4],[18,18],[46,39],[12,24]
            //  0     1     2        3      4       5       6       7       8       9     10     11     12      13
        ];
        assert_eq!(Solution::get_order(cases), vec![6,1,2,9,4,10,0,11,5,13,3,8,12,7]);
    }
}

// 6   2,15  6
// 17  16,9  1
// 26  21,10  2
// 36  33,6   9
// 42  37,4   4
// 46  45,4   10
// 50  19,13   0
