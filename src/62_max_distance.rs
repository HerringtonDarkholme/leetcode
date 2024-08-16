impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let (mut min, mut max) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        let mut ret = 0;
        for array in arrays.into_iter().skip(1) {
            let diff1 = array[array.len() - 1] - min;
            let diff2 = max - array[0];
            ret = ret.max(diff1).max(diff2);
            min = min.min(array[0]);
            max = max.max(array[array.len() - 1]);
        }
        ret
    }
}
