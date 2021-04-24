struct Solution;
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let xor1 = arr1.into_iter().fold(0, |a, b| a ^ b);
        let xor2 = arr2.into_iter().fold(0, |a, b| a ^ b);
        xor1 & xor2
    }

    pub fn get_xor_sum_silly(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in arr1 {
            for &j in arr2.iter() {
                sum ^= i & j;
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test() {
        let cases = vec![
            (
                vec![1,2,3], vec![6,5],
            ),
            (
                vec![12], vec![4],
            ),
            (
                vec![0], vec![1,2,3,4,5,6,7],
            ),
            (
                vec![1], vec![1,1],
            ),
            (
                vec![3], vec![1,1]
            )
        ];
        for (a1, a2) in cases {
            assert_eq!(Solution::get_xor_sum(a1.clone(), a2.clone()), Solution::get_xor_sum_silly(a1, a2))
        }
    }
}
