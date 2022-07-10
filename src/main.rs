#[macro_use]
mod util;
#[path = "./recursive_remove_adjacent_duplicates.rs"]
pub mod remove_adj;

fn main() {}

#[test]
fn test() {
    // assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
    // assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    assert_eq!(Solution::count_paths(nested![[1,1],[3,4]]), 8);
    assert_eq!(Solution::count_paths(nested![[1,2]]), 3);
    assert_eq!(Solution::count_paths(nested![
            [1,2,4,4,3,2,3],
            [2,8,3,4,5,6,9],
            [3,4,1,2,3,6,8],
    ]), 105);
}

struct Solution;

