impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() { return vec![new_interval] }
        let start_idx = match intervals.binary_search_by_key(&new_interval[0], |i| i[1]) {
            Ok(id) => id,
            Err(id) => id,
        };
        let end_idx = match intervals.binary_search_by_key(&new_interval[1], |i| i[0]) {
            Ok(id) =>  id + 1,
            Err(id) => id,
        };
        let mut ret = vec![];
        ret.extend(intervals[0..start_idx].iter().cloned());
        let start = if start_idx < intervals.len() {
            intervals[start_idx][0].min(new_interval[0])
        } else {
            new_interval[0]
        };
        let end = if end_idx > 0 { 
            intervals[end_idx - 1][1].max(new_interval[1])
        } else { new_interval[1] };
        ret.push(vec![start, end]);
        ret.extend(intervals[end_idx..].iter().cloned());
        ret
    }
}
