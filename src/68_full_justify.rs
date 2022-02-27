struct Solution;
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ret = vec![];
        let max = max_width as usize;
        let mut i = 0;
        while i < words.len() {
            let next = divide(&words, i, max);
            let line = print_one_line(&words[i..next], max, next == words.len());
            ret.push(line);
            i = next;
        }
        debug(&ret);
        ret
    }
}

fn divide(words: &[String], start: usize, max: usize) -> usize {
    if words.len() <= start {
        return words.len();
    }
    let mut i = start;
    let mut len = words[i].len();
    while len < max && i + 1 < words.len() {
        let next = &words[i + 1];
        if len + 1 + next.len() > max {
            break;
        }
        len += 1 + next.len();
        i += 1;
    }
    i + 1
}

fn print_one_line(words: &[String], max: usize, is_last: bool) -> String {
    if is_last {
        let first: String = words.join(" ");
        return format!("{first}{}", " ".repeat(max - first.len()));
    }
    if words.len() == 1 {
        return format!("{}{}", words[0], " ".repeat(max - words[0].len()));
    }
    let n: usize = words.iter().map(|s| s.len()).sum();
    let spaces = (max - n) / (words.len() - 1);
    let space = " ".repeat(spaces);
    let mut remain = (max - n) % (words.len() - 1);
    let mut s = String::new();
    for (c, w) in words.iter().enumerate() {
        s.push_str(w);
        if c + 1 == words.len() {
            continue;
        }
        s.push_str(&space);
        if remain > 0 {
            remain -= 1;
            s.push(' ');
        }
    }
    s
}

fn debug(a: &Vec<String>) {
    println!("---------------");
    for s in a {
        println!("|{s}|");
    }
}
