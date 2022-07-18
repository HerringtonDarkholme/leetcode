impl Solution {
    pub fn is_n_straight_hand(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::BTreeMap;
        if nums.len() % (k as usize) != 0 {
            return false;
        }
        let mut counter = BTreeMap::new();
        for &n in &nums {
            *counter.entry(n).or_insert(0) += 1;
        }
        while let Some((&n, &base)) = counter.iter().next() {
            for i in 0..k {
                let mut remove = false;
                if let Some(cnt) = counter.get_mut(&(n + i)) {
                    if *cnt < base {
                        return false;
                    }
                    *cnt -= base;
                    remove = *cnt == 0;
                } else {
                    return false;
                }
                if remove {
                    counter.remove(&(n + i));
                }
            }
        }
        true
    }
}

// Count number of different cards to a map c
// Loop from the smallest card number.
// Everytime we meet a new card i, we cut off i - i + W - 1 from the counter.
