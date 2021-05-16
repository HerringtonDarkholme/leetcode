use std::collections::{
    VecDeque
};

struct MKAverage {
    order: VecDeque<i32>,
    rank: Vec<i32>,
    offset: usize,
    max: usize,
    skip: usize,
    sum: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {

    fn new(m: i32, k: i32) -> Self {
        let max = m as usize;
        let skip = k as usize;
        assert!(k*2 < m, "1 <= k*2 < m");
        MKAverage {
            order: VecDeque::with_capacity(max),
            rank: Vec::with_capacity(max),
            offset: 0,
            sum: 0,
            max, skip,
        }
    }
    
    fn add_element(&mut self, num: i32) {
        let k = self.skip;
        let m = self.max;
        if self.order.len() >= m {
            let removed = self.order.pop_front().expect("non empty");
            let r = self.rank.binary_search(&removed).expect("inconsistent rank/order");
            if r < k {
                self.sum += self.rank[m-k] - self.rank[k];
            } else if r < m - k {
                self.sum += self.rank[m-k] - removed;
            }
            self.rank.remove(r);
        }
        self.order.push_back(num);
        let r = match self.rank.binary_search(&num) {
            Ok(r) => r,
            Err(r) => r,
        };
        self.rank.insert(r, num);
        if r < k {
            if self.rank.len() > k {
                self.sum += self.rank[k];
            }
            if self.rank.len() > m - k {
                self.sum -= self.rank[m - k];
            }
        } else if r < m - k {
            self.sum += num;
            if self.rank.len() > m - k {
                self.sum -= self.rank[m - k];
            }
        }
    }
    
    fn calculate_mk_average(&self) -> i32 {
        let k = self.skip;
        let m = self.max;
        if self.rank.len() < m {
            -1
        } else {
            self.sum / (m - 2*k) as i32
        }
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */
