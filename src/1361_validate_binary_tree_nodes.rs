impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut set = vec![false; n];
        for i in 0..n {
            let l = left_child[i];
            let r = right_child[i];
            if l >= 0 {
                set[l as usize] = true;
            }
            if r >= 0 {
                set[r as usize] = true;
            }
        }
        let mut root = -1;
        for i in 0..n {
            if set[i] {
                continue;
            }
            if root >= 0 {
                return false;
            }
            root = i as i32;
            set[i] = true;
        }
        if root < 0 {
            return false;
        }
        dfs(root as usize, &left_child, &right_child, &mut set) && set.into_iter().all(|c| !c)
    }
}
fn dfs(root: usize, left: &[i32], right: &[i32], set: &mut [bool]) -> bool {
    if !set[root] {
        return false;
    }
    set[root] = false;
    (
        left[root] < 0 || dfs(left[root] as usize, left, right, set)
    ) && (
        right[root] < 0 || dfs(right[root] as usize, left, right, set)
    )
}
