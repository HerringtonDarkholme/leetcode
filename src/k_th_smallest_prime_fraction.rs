use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::cmp::Reverse;

#[derive(Eq, PartialEq, Debug)]
struct MyRational {
    n: i32,
    d: i32
}

impl Ord for MyRational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.n * other.d).cmp(&(self.d * other.n))
    }
}

impl PartialOrd for MyRational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 「素数」を数えて落ち着くんだ
impl Solution {
    pub fn kth_smallest_prime_fraction(a: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        let n = a.len() - 1;
        heap.push((Reverse(MyRational{n: a[0], d: a[n]}), 0, n));
        set.insert((0, n));
        k -= 1;
        while k > 0 {
            let (r, n, m) = heap.pop().unwrap();
            if !set.contains(&(n+1, m)) {
                heap.push((Reverse(MyRational{n:a[n + 1], d:a[m]}), n+1, m));
                set.insert((n+1, m));
            }
            if !set.contains(&(n, m-1)) {
                heap.push((Reverse(MyRational{n: a[n], d:a[m-1]}), n, m -1));
                set.insert((n, m-1));
            }
            k -= 1;
        }
        let (_, n, m) = heap.pop().unwrap();
        vec![a[n], a[m]]
    }
}
