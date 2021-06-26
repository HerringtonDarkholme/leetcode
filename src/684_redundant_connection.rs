struct  Solution;

struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl DSU {
    fn new(size: usize) -> Self {
        DSU {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (xr, yr) = (self.find(x), self.find(y));
        if xr == yr {
            return false
        }
        if self.rank[xr] < self.rank[yr] {
            self.parent[xr] = yr;
        } else if self.rank[xr] > self.rank[yr] {
            self.parent[yr] = xr
        } else {
            self.parent[yr] = xr;
            self.rank[xr] += 1;
        }
        true
    }
}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = DSU::new(edges.len() + 1);
        for edge in edges {
            if !dsu.union(edge[0] as usize, edge[1] as usize) {
                return edge
            }
        }
        unreachable!()
    }
}
