impl Solution {
    pub fn count_and_say(n: i32) -> String {
        count(n)
    }
}

fn count(n: i32) -> String {
    let mut v = vec![1];
    for i in 1..n {
        let mut next = vec![];
        let mut last = v[0];
        let mut cnt = 1;
        for j in 1..v.len() {
            if last == v[j] {
                cnt += 1;
            } else {
                next.push(cnt);
                next.push(last);
                cnt = 1;
                last = v[j];
            }
        }
        next.push(cnt);
        next.push(last);
        v = next;
    }
    v.into_iter().map(|c| c.to_string()).collect()
}
