use std::rc::Rc;
use std::collections::HashMap;

impl Solution {
    pub fn restore_array(pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();
        for pair in pairs {
            let mut left = get_arr(&mut map, pair[0]);
            let mut right = get_arr(&mut map, pair[1]);
            // merge two
            right.reverse();
            left.extend(right);
            let v = Rc::new(left);
            let left = v[0];
            let right = *v.last().unwrap();
            map.insert(left, v.clone());
            map.insert(right, v);
        }
        let mut values = map.into_values();
        values.next();
        Rc::try_unwrap(values.next().unwrap()).unwrap()
    }
}

fn get_arr(map: &mut HashMap<i32, Rc<Vec<i32>>>, end: i32) -> Vec<i32> {
    let v = match map.remove(&end) {
        Some(v) => v,
        None => return vec![end],
    };
    let need_reverse = if v[0] == end {
        let other = *v.last().unwrap();
        map.remove(&other);
        true
    } else {
        map.remove(&v[0]);
        false
    };
    let mut v = Rc::try_unwrap(v).unwrap();
    if need_reverse {
        v.reverse();
    }
    v 
}
