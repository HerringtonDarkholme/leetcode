impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        pancake_sort(&mut arr).into_iter().map(|i| i as i32).collect()
    }
}
/// 4321 123

fn pancake_sort(arr: &mut [i32]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }
    let mut max_i = 0;
    let mut max = i32::MIN;
    for i in 0..arr.len() {
        if arr[i] > max {
            max_i = i;
            max = arr[i];
        }
    }
    let len = arr.len();
    // largest element alread in the back
    if max_i == arr.len() - 1 {
        return pancake_sort(&mut arr[..len-1]);
    }
    let mut ret = vec![
        max_i + 1, // to first element
        arr.len(), // to back
    ];
    reverse(arr, max_i);
    reverse(arr, arr.len() - 1);
    let remaining = pancake_sort(&mut arr[..len - 1]);
    ret.extend(remaining);
    ret
}

fn reverse(arr: &mut [i32], mut end: usize) {
    let mut i = 0;
    let mut j = end;
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}
