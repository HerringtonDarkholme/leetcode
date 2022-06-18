impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut t = threshold as usize;
        let mut parents: Vec<_> = (0..n).collect();
        let mut rank = vec![0; n];
        t += 1;
        let mut i = t;
        while i<=n {
            let mut m = 1;
            while (i*m <= n) {
                union(i - 1, i*m - 1, &mut parents, &mut rank);
                m += 1;
            }
            i += 1;
        }
        queries.into_iter().map(|v| {
            let i = v[0] as usize - 1;
            let j = v[1] as usize - 1;
            find(i, &parents) == find(j, &parents)
        }).collect()
    }
}

fn union(x: usize, y: usize, parents: &mut [usize], rank: &mut [usize]) {
    let px = find(x, parents);
    let py = find(y, parents);
    if rank[px] < rank[py] {
        parents[px] = py;
        return;
    }
    if rank[px] == rank[py] {
        rank[px] += 1;
    }
    parents[py] = parents[px];
}

fn find(mut x: usize, parents: &[usize]) -> usize {
    while x != parents[x] {
        x = parents[x];
    }
    x
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}
