impl Solution {
    pub fn can_complete_circuit(mut gas: Vec<i32>, mut cost: Vec<i32>) -> i32 {
        let len = gas.len();
        gas.extend(gas.clone());
        cost.extend(cost.clone());
        let abs: Vec<_> = gas.into_iter().zip(cost.into_iter()).map(|(g, c)| g - c).collect();
        let mut sum = 0;
        let mut i = 0;
        while i < len {
            let mut j = i;
            loop {
                sum += abs[j];
                if sum < 0 {
                    i = j + 1;
                    sum = 0;
                    break;
                }
                j += 1;
                if j - i == len {
                    return i as i32;
                }
            }
        }
        -1
    }
}
