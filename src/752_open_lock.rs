
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target: i32 = target.parse().unwrap();
        let mut seen = vec![false; 10000];
        for d in deadends {
            seen[d.parse::<usize>().unwrap()] = true;
        }
        if seen[0] {
            return -1
        }
        seen[0] = true;
        let mut frontier = vec![0];
        let mut steps = 0;
        while !frontier.is_empty() {
            let mut next = vec![];
            for f in frontier {
                if f == target {
                    return steps
                }
                for i in &[1,10,100,1000] {
                    for &j in &[1, -1] {
                        let d = (f / i) % 10;
                        let n = if d + j > 9 {
                            f - 9 * i
                        } else if d + j < 0 {
                            f + 9 * i
                        } else {
                            f + i * j
                        };
                        if seen[n as usize] {
                            continue;
                        }
                        seen[n as usize] = true;
                        next.push(n);
                    }
                }
            }
            frontier = next;
            steps += 1;
        }
        -1
    }
}
