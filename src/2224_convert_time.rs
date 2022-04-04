
impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let mut diff = convert_time(correct).unwrap() - convert_time(current).unwrap();
        let hour = diff / 60;
        diff %= 60;
        let quater = diff / 15;
        diff %= 15;
        let five = diff / 5;
        diff %= 5;
        hour + quater + five + diff
    }
}

fn convert_time(current: String) -> Option<i32> {
    let mut splits = current.split(":");
    let hour: i32 = splits.next()?.parse().ok()?;
    let mins: i32 = splits.next()?.parse().ok()?;
    Some(hour * 60 + mins)
}
