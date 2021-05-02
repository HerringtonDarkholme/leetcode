impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut num: Vec<_> = num.chars().collect();
        num.reverse();
        let mut changed = nth_wonderful(&num, k);
        find_swaps(&mut changed, &num)
    }
}

fn nth_wonderful(num: &Vec<char>, mut k: i32) -> Vec<char> {
    let mut ret = num.clone();
    let exp = "Input number should have next wonderful!";
    while k != 0 {
        let i = (1..ret.len())
            .filter(|&i| ret[i - 1] > ret[i])
            .next()
            .expect(exp);
                
        let j = (0..i)
            .filter(|&j| ret[j] > ret[i])
            .next()
            .expect(exp);
        ret.swap(i, j);
        ret[0..i].reverse();
        k -= 1;
    }
    ret
}
fn find_swaps(source: &mut [char], target: &[char]) -> i32 {
    let mut r = 0;
    for i in 0..source.len() {
        if source[i] == target[i] {
            continue;
        }
        let j = (i..source.len())
            .filter(|&j| source[j] == target[i])
            .next()
            .expect("Input should have next wonderful!");
        source.copy_within(i..j, i+1);
        source[i] = target[i];
        r += j - i;
    }
    r as i32
}
