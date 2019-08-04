// mod zig_zag;

#[macro_use]
mod util;
pub mod max_sum_of_three_subarrays;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
