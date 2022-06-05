impl Solution {
    pub fn array_change(mut nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut idx = std::collections::HashMap::new();
        for i in 0..nums.len() {
            idx.insert(nums[i], i);
        }
        for op in operations {
            let original = op[0];
            let swapped = op[1];
            let id = idx[&original];
            idx.remove(&original);
            idx.insert(swapped, id);
            nums[id] = swapped;
        }
        nums
    }
}
