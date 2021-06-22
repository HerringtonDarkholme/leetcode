impl Solution {
    pub fn read_binary_watch(l: i32) -> Vec<String> {
        let mut ret = vec![];
        for i in 0..=l.min(3) {
            for h in read_hour(i) {
                for m in read_min(l - i) {
                    ret.push(format!("{}:{:02}", h, m));
                }
            }
        }
        ret
    }
}
fn read_gen(i: i32, b: i32) -> Box<dyn Iterator<Item=i32>> {
    if i == 0 { return Box::new(0..1) }
    Box::new(read_gen(i - 1, b).flat_map(move |j| {
        (0..b).filter_map(move |i| {
            let c = 1 << i;
            if c & j == 0 && c > j {
                Some(c | j)
            } else {
                None
            }
        })
    }))
}

fn read_hour(i: i32) -> impl Iterator<Item=i32> {
    read_gen(i, 4).filter(|&h| h < 12)
}
fn read_min(i: i32) -> impl Iterator<Item=i32> {
    read_gen(i, 6).filter(|&m| m < 60)
}
