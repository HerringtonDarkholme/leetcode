pub struct Solution;

impl Solution {
    pub fn three_equal_parts(a: Vec<i32>) -> Vec<i32> {
        // observation: all three parts have equal ones
        let ones = a.iter().filter(|&&i| i == 1).collect::<Vec<_>>().len();
        if ones % 3 != 0 {
            return vec![-1, -1]
        }
        if ones == 0 {
            return vec![0, a.len() as i32 - 1]
        }
        let target = ones / 3;
        // but enough ones cannot guarantee the number
        let start2 = find_number(&a, target);
        // then just find number from start, find two number and check if
        // its end exceed start2
        if let Some(end1) = find_end(&a, 0, start2) { if let Some(end2) = find_end(&a, end1 + 1, start2) {
                if end2 < start2 {
                    vec![end1 as i32, end2 as i32 + 1]
                } else {
                    vec![-1, -1]
                }
            } else {
                vec![-1, -1]
            }
        } else {
            vec![-1, -1]
        }

    }

}

// because leading zeroes are trivial, we cannot find number from beginning
// instead we find it backward from end, since trailing zeroes are significant
fn find_number(a: &Vec<i32>, target: usize) -> usize {
    let mut i = a.len();
    let mut seen = 0;
    while i > 0 {
        if a[i - 1] == 1 {
            seen += 1;
        }
        i -= 1;
        if seen == target {
            return i
        }
    }
    panic!("Impossible!")
}

fn find_end(a: &Vec<i32>, mut s1: usize, mut s2: usize) -> Option<usize> {
    // ignore leading zero
    while a[s1] == 0 {
        s1 += 1;
    }
    // we can guarantee s1 is less than s2
    while s2 < a.len() {
        if a[s1] != a[s2] {
            return None
        }
        s1 += 1;
        s2 += 1;
    }
    Some(s1 - 1)
}
