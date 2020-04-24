impl Solution {
    pub fn max_satisfaction(mut dishes: Vec<i32>) -> i32 {
        satisfaction(dishes)
    }
}

fn satisfaction(mut dishes: Vec<i32>) -> i32 {
    dishes.sort();
    let positive_sum: i32 = dishes.iter().filter(|&&n| n > 0).sum();
    let negative: Vec<i32> = dishes.iter().filter(|&&n| n < 0).cloned().collect();
    let mut i = 0;
    loop {
        let diff = negative[i..].iter().sum::<i32>().abs();
        if  diff > positive_sum {
            i += 1;
        } else {
            break;
        }
    }
    dishes[i..].iter().enumerate().map(|(i, &k)| (i as i32 + 1) * k).sum()
}
