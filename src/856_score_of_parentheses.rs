impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        score(s)
    }
}
fn score(s: String) -> i32 {
    // a stack maintaining current level score
    let mut scores = vec![0];
    for c in s.chars() {
        if c == '(' {
            // add one level
            scores.push(0);
        } else {
            // reduce one level
            let curr = scores.pop().unwrap();
            let prev = scores.pop().unwrap();
            scores.push(prev + (curr * 2).max(1));
        }
    }
    scores[0]
}

// ( ( () () ) )
// 0 [8]
