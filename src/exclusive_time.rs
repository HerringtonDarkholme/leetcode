impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut times = vec![0; n as usize];
        let mut exec = vec![];
        let mut start = 0;
        for log in logs {
            let parts: Vec<&str> = log.split(":").collect();
            let id: usize = parts[0].parse().unwrap();
            let time: i32 = parts[2].parse().unwrap();
            let action = parts[1];
            if action == "start" {
                if !exec.is_empty() {
                    let last = *exec.last().unwrap();
                    times[last] += time - start;
                }
                exec.push(id);
                start = time;
            } else {
                assert_eq!(exec.pop().unwrap(), id);
                times[id] += time - start + 1;
                start = time + 1;
            }
        }
        times
    }
}
