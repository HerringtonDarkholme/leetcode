pub struct  Solution;

use std::collections::BTreeMap;

struct UF {
    parent: Vec<usize>
}

impl UF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        if p != x {
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> (usize, usize) {
        let px = self.find(x);
        let py = self.find(y);
        self.parent[px] = py;
        (px, py)
    }
}

impl Solution {
    pub fn matrix_rank_transform(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut rank = vec![0; m + n];
        let mut map = BTreeMap::new();
        for i in 0..m {
            for j in 0..n {
                map.entry(matrix[i][j]).or_insert_with(|| vec![]).push((i, j));
            }
        }
        for coords in map.values() {
            let mut uf = UF::new(m + n);
            let mut rank2 = rank.clone();
            for (i, j) in coords.iter().cloned() {
                let (pi, pj) = uf.union(i, j + m);
                rank2[pj] = rank2[pj].max(rank2[pi]);

            }
            for (i, j) in coords.iter().cloned() {
                let v = rank2[uf.find(i)] + 1;
                rank[i] = v;
                rank[j + m] = v;
                matrix[i][j] = v as i32;
            }
        }
        matrix
    }
}
