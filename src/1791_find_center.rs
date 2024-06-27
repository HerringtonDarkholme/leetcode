impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let a = edges[0][0];
        if edges[1].contains(&a){
            a
        } else {
            edges[0][1]
        }
    }
}
