const N: usize = 26;
fn c_to_i(c: char) -> usize {
    (c as usize) - ('a' as usize)
}
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut a = vec![N; N];
        for (j, c) in order.chars().enumerate() {
            let i = c_to_i(c);
            a[i] = j;
        }
        let mut b: Vec<_> = s.chars().collect();
        b.sort_by_key(|&c| a[c_to_i(c)]);
        b.into_iter().collect()
    }
}
