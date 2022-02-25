impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (a, b) = parse(num1);
        let (c, d) = parse(num2);
        let real = (a * c - b * d);
        let img = (a * d + b * c);
        format!("{real}+{img}i")
    }
}
fn parse(num: String) -> (i32, i32) {
    let mut nums = num.split('+');
    let a = nums.next().unwrap().parse().unwrap();
    let b = nums.next().unwrap();
    let b = b[..b.len() - 1].parse().unwrap();
    (a, b)
}
// (a+bi) * (c + di) = ac+adi + bci - bd
