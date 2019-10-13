pub struct Solution;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by_key(|v| (v[0], v[1]));
        let mut ret = vec![vec![]; people.len()];
        let mut index = (0..people.len()).collect::<Vec<_>>();
        let mut last = 0;
        for i in 0..people.len() {
            let person = &people[i];
            if i == 0 {
                let ind = person[1] as usize;
                ret[index[ind]] = person.clone();
                index.remove(ind);
                continue;
            }
            last = if person[0] == people[i - 1][0] {
                last + 1
            } else { 0 };
            let ind = person[1] as usize - last;
            ret[index[ind]] = person.clone();
            index.remove(ind);
        }
        ret
    }
}
