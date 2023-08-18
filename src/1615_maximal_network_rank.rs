impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut connection = vec![vec![0; n as usize]; n as usize];
        let mut ranks = vec![0; n as usize];
        for road in roads {
            let (a, b) = if road[0] < road[1] { (road[0], road[1]) } else { (road[1], road[0]) };
            ranks[a as usize] += 1;
            ranks[b as usize] += 1;
            connection[a as usize][b as usize] = 1;
        }
        let mut max = 0;
        for i in 0..n as usize {
            for j in 0..i {
                max = max.max(ranks[i] + ranks[j] - connection[j][i]);
            }
        }
        max
    }
}
