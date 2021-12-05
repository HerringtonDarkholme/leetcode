impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        combination(digits)
    }
}

fn combination(mut digits: String) -> Vec<String> {
    let mut strings = vec![vec![]];
    let chars = vec![
      vec!['a', 'b', 'c'],  
      vec!['d', 'e', 'f'],  
      vec!['g', 'h', 'i'],  
      vec!['j', 'k', 'l'],  
      vec!['m', 'n', 'o'],  
      vec!['p', 'q', 'r', 's'],  
      vec!['t', 'u', 'v'],  
      vec!['w', 'x', 'y', 'z'],
    ];
    for d in digits.bytes() {
        let mut next = vec![];
        let i = (d - b'2') as usize;
        for s in strings {
            for &c in chars[i].iter() {
                let mut n = s.clone();
                n.push(c);
                next.push(n);
            }
        }
        strings = next;
    }
    strings.into_iter().map(|v| v.into_iter().collect()).collect()
}
