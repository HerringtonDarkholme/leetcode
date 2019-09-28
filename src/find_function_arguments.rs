/*
 * Given a number z ∈ N and a function f(x, y) ∈ N, where x ∈ N and y ∈ N.
Function f(x, y) is black box and unknown but we can pass any x and y to function and get f(x, y) value. f(x, y) is strictly increasing:
*/

type F = fn(usize, usize) -> usize;

fn find_function_arguments(f: F, z: usize) -> Vec<(usize, usize)> {
    let xmin = bsearch_x_min(z, f);
    let xmax = bsearch_x_max(z, f);
    if xmin == 0 || xmax == 0 {
        return vec![]
    }
    (xmin..=xmax).filter_map(|x| {
        let y = bsearch(x, z, f);
        if y != 0 {
            Some((x, y))
        } else {
            None
        }
    }).collect()
}

fn bsearch(x: usize, z: usize, f: F) -> usize {
    let mut ymin = 1;
    let mut ymax = z;
    while ymin <= ymax {
        let mid = ymin + ((ymax - ymin) >> 1);
        let t = f(x, mid);
        if t == z {
            return mid
        } else if t > z {
            ymax = mid - 1;
        } else {
            ymin = mid + 1;
        }
    }
    0
}

fn bsearch_x_min(z: usize, f: F) -> usize {
    let mut xmin = 1;
    let mut xmax = z;
    while xmin <= xmax {
        let mid = xmin + (xmax - xmin) / 2;
        let t = f(mid, z);
        let t1 = f(mid + 1, z);
        if t == z {
            return mid
        } else if t1 == z {
            return mid + 1
        } else if t > z {
            if mid == 1 {
                return 1;
            }
            xmax = mid - 1;
        } else if t1 < z {
            xmin = mid + 1;
        } else {
            return mid + 1;
        }
    }
    0
}

fn bsearch_x_max(z: usize, f: F) -> usize {
    let mut xmin = 1;
    let mut xmax = z;
    while xmin <= xmax {
        let mid = xmin + (xmax - xmin) / 2;
        let t = f(mid, 1);
        let t1 = f(mid + 1, 1);
        if t == z {
            return mid
        } else if t1 == z {
            return mid + 1
        } else if t > z {
            xmax = mid - 1;
        } else if t1 < z {
            xmin = mid + 1;
        } else {
            return mid;
        }
    }
    0
}

fn add(x: usize, y: usize) -> usize {
    x + y
}
fn square(x: usize, y: usize) -> usize {
    x * x + y * y
}

#[test]
fn test() {
    for (f, z) in vec![
        // (add, 5),
        (square, 25),
        (square, 5),
    ] {
        println!("{:?}", find_function_arguments(f, z));
    }
    panic!("prin");
}
