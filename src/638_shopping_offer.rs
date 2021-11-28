use std::collections::HashMap;
impl Solution {
    pub fn shopping_offers(price: Vec<i32>, mut special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        cache.insert(0, 0);
        search(&price, &special, needs, &mut cache)
    }
}

fn search(price: &Vec<i32>, options: &Vec<Vec<i32>>, needs: Vec<i32>, cache: &mut HashMap<i32, i32>) -> i32 {
    let hash = hash_needs(&needs);
    if let Some(r) = cache.get(&hash) {
        return *r;
    }
    let mut min = {
        let mut sum = 0;
        for i in 0..price.len() {
            sum += price[i] * needs[i];
        }
        sum
    };
    for opt in options.iter() {
        if opt[needs.len()] > min {
            continue;
        }
        let mut further = false;
        let mut nds = needs.clone();
        for i in 0..nds.len() {
            if nds[i] < opt[i] {
                further = false;
                break;
            }
            further = true;
            nds[i] = 0.max(nds[i] - opt[i]);
        }
        if further {
            min = min.min(search(price, options, nds, cache) + opt[needs.len()]);
        }
    }
    cache.insert(hash, min);
    min
}

fn hash_needs(needs: &Vec<i32>) -> i32 {
    let mut base = 1;
    let mut hash = 0;
    for &n in needs.iter() {
        hash += base * n;
        base *= 11;
    }
    hash
}
