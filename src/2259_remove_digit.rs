
impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let nums: Vec<_> = number.chars().collect();
        let mut largest = None;
        for i in 0..nums.len() {
            if nums[i] != digit {
                continue;
            }
            if largest.is_none() {
                largest = Some(i);
            } else {
                let last = *largest.as_ref().unwrap();
                if nums[last] >= nums[last + 1] {
                    largest = Some(i);
                }
            }
        }
        let largest = largest.unwrap();
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, c)| if i == largest { None } else { Some(c) })
            .collect()
    }
}
