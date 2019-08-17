// mod zig_zag;

#[macro_use]
mod util;
pub mod count_digit_one;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
