impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zip: Vec<_> = names.into_iter().zip(heights.into_iter()).collect();
        zip.sort_by_key(|a| -a.1);
        zip.into_iter().map(|a| a.0).collect()
    }
}
