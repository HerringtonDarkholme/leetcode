impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let colors = colors.as_bytes();
        let mut a = 0;
        let mut b = 0;
        for i in 1..(colors.len() - 1) {
            if colors[i] == b'A' && colors[i - 1] == b'A' && colors[i + 1] == b'A'{
                a += 1;
            }
            if colors[i] == b'B' && colors[i - 1] == b'B' && colors[i + 1] == b'B' {
                b += 1;
            }
        }
        a > b
    }
}
