impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut i = 0;
        let mut j = people.len() - 1;
        let mut ret = 0;
        while i < j {
            ret += 1;
            if people[i] + people[j] <= limit {
                i += 1;
            }
            j -= 1;
        }
        // left only one boat
        if i == j {
            ret += 1;
        }
        ret
    }
}
//   i          j
// [ 1 ,2 3 4 5 6 ]
// if n[i] + n[j] > limit j--
// if n[i] + n[j] <= limit i++, j--
// i == j
