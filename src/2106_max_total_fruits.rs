impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let (left, origin, right) = pre(fruits, start_pos, k);
        let leftmost = left.last().map_or(0, |s| s.1);
        let rightmost = right.last().map_or(0, |s| s.1);
        let left_return = return_path(&left, &right, k);
        let right_return = return_path(&right, &left, k);
        leftmost.max(rightmost).max(left_return).max(right_return) + origin
    }
}

fn pre(fruits: Vec<Vec<i32>>, start: i32, k: i32) -> (Vec<(i32, i32)>, i32, Vec<(i32, i32)>) {
    let mut left = vec![];
    let mut right = vec![];
    let mut origin = 0;
    let mut acc = 0;
    for fruit in fruits.iter() {
        let pos = fruit[0];
        let num = fruit[1];
        if pos == start {
            origin = num;
            continue;
        }
        if pos < start {
            continue;
        }
        if pos > start + k {
            break;
        }
        acc += num;
        right.push((pos - start, acc));
    }
    acc = 0;
    for fruit in fruits.iter().rev() {
        let pos = fruit[0];
        let num = fruit[1];
        if pos >= start {
            continue;
        }
        if pos < start - k {
            break;
        }
        acc += num;
        left.push((start - pos, acc));
    }
    (left, origin, right)
}

fn find(fruits: &[(i32, i32)], steps: i32) -> i32 {
    match fruits.binary_search(&(steps, i32::MAX)) {
        Ok(f) => fruits[f].0,
        Err(f) => if f > 0 { fruits[f - 1].1 } else { 0 }
    }
}

fn return_path(left: &[(i32, i32)], right: &[(i32, i32)], k: i32) -> i32 {
    let mut max = 0;
    for i in 0..left.len() {
        let (step, num) = left[i];
        let right_max = find(right, k - step * 2);
        max = max.max(num + right_max);
    }
    max
}
