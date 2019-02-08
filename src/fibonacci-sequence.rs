struct Solution {}

static mut CACHE: [i32; 31] = [0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

impl Solution {
    pub fn fib(n: i32) -> i32 {
        // why Rust if using unsafe?
        unsafe {
            Solution::aux(n as usize)
        }
    }

    unsafe fn aux(n: usize) -> i32 {
        if n == 0 {
            return 0
        }
        if CACHE[n] != 0 {
            return CACHE[n]
        }
        let ret = Solution::aux(n - 1) + Solution::aux(n - 2);
        CACHE[n] = ret;
        ret
    }
}

fn main() {

    let tests = vec!(0, 1, 2, 3, 4, 30);
    for test in tests {
        println!("{} {}", test, Solution::fib(test));
    }
}
