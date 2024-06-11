impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut sorter = vec![1002; 1001];
        for (i, n) in arr2.into_iter().enumerate() {
            sorter[n as usize] = i;
        }
        let mut arr1: Vec<_> = arr1.into_iter().map(|n| (sorter[n as usize], n)).collect();
        arr1.sort();
        arr1.into_iter().map(|n| n.1).collect()
    }
}
