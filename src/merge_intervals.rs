pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let find_left = Solution::find_left;
        let find_right = Solution::find_right;
        for i in intervals {
            let l = find_left(&ret, i[0]);
            let r = find_right(&ret, i[1]);
            match (l, r) {
                (_, None) => {
                    ret.insert(0, (i[0], i[1]));
                    continue;
                },
                (None, _) => {
                    ret.push((i[0], i[1]));
                    continue;
                },
                _ => (),
            }
            let l = l.unwrap();
            let r= r.unwrap();
            if l > r {
                ret.insert(l, (i[0], i[1]));
                continue;
            }
            let min = std::cmp::min(ret[l].0, i[0]);
            let max = std::cmp::max(ret[r].1, i[1]);
            ret.splice(l..=r, vec![]);
            ret.insert(l, (min, max));
        }
        ret.iter().map(|k| vec![k.0, k.1]).collect()
    }
    fn find_left(ret: &[(i32, i32)], t: i32) -> Option<usize> {
        for (i, v) in ret.iter().enumerate() {
            if v.1 >= t {
                return Some(i)
            }
        }
        None
    }
    fn find_right(ret: &[(i32, i32)], t: i32) -> Option<usize> {
        for (i, v) in ret.iter().enumerate().rev() {
            if v.0 <= t {
                return Some(i)
            }
        }
        None
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (nested![[1,3], [2,6],[8,10],[15,18]], nested![[1,6],[8,10],[15,18]]),
        (nested![[2,6],[1,3], [15,18], [8,10]], nested![[1,6],[8,10],[15,18]]),
        (nested![[1,4],[4,5]], nested![[1,5]]),
        (nested![], nested![]),
        (nested![[1,4],[0,0]], nested![[0,0], [1,4]]),
        (nested![[2,3],[5,5],[2,2],[3,4],[3,4]], nested![[2, 4], [5, 5]]),
        (nested![[1,5], [2,8], [2,9], [3,10], [5,8], [6,6], [4,10], [8, 11]], nested![[1, 11]]),
    ], Solution::merge);
}
