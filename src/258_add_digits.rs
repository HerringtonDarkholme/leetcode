impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        if num == 0 { 
            0 
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
}
