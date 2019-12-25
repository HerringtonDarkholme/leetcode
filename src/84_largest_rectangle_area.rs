pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        aux(heights)
    }
}

// intuition: compute all largest area ends with index i.
// largest rects are areas with height of current histogram
// or areas with height of previous histogram shorter than current one
// N.B. drawing a graph helps understanding
// this is typical sequence of partial order. let's use mono-stack
// we will compute max area when element get popped out from stack
fn aux(mut heights: Vec<i32>) -> i32 {
    let mut max = 0;
    heights.push(0); // helper elem to flush stack
    let mut stack = Vec::with_capacity(heights.len());
    for (i, &h) in heights.iter().enumerate() {
        while !stack.is_empty() {
            let last_index = *stack.last().unwrap();
            if h > heights[last_index] {
                break;
            }
            stack.pop(); // pop out and compute area
            let before = stack.last().map(|&i| i + 1).unwrap_or(0);
            max = max.max(heights[last_index] * (i - before) as i32);
        }
        stack.push(i);
    }
    max
}
// alternatively, we can compute the nearest left shorter element
// and nearest right shorter element. then compute the max area with height of histo[i]
