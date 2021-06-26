pub struct DSU {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>
}

impl DSU {
    pub fn new(size: usize) -> Self {
        DSU {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
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
