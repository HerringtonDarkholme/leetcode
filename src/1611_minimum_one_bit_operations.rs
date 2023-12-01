use std::collections::HashMap;
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut cache = HashMap::new();
        to_zero(n, &mut cache)
    }
}
// 1 0 1 0
fn to_zero(num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&ret) = cache.get(&num) {
        return ret;
    }
    // if base case: return
    if num <= 0 { return num; }
    if num & (num - 1) == 0 { return num * 2 - 1; } // shortcut
    let mut ret = 0;
    // flip_first_one + to_zero(n[1:])
    for i in (1..32).rev() {
        if num & (1 << i) != 0 {
            ret = flip_first_one(i, num, cache) + (1 << i) - 1;
            break;
        }
    }
    cache.insert(num, ret);
    ret
}
fn flip_first_one(bit: i32, num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    assert!(num & (1<<bit) != 0);
    // to_leading_one(n[1:]) + 1 // flip
    to_leading_one(bit-1, num ^ (1 << bit), cache) + 1
}
fn to_leading_one(bit: i32, num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if bit < 1 { // 10
        return if num == 1 { 0 } else { 1 }
    }
    // if n[0] is 1: to_zero(n[1:])
    if num & (1 << bit) != 0 {
        to_zero(num ^ (1 << bit), cache)
    } else { // if n[0] is 0
        to_leading_one(bit - 1, num, cache) //0100000
        + 1                                 //1100000
        + (1 << bit) - 1                    //1000000
    }      
}  
