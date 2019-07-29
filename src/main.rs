// mod zig_zag;

#[macro_use]
mod util;
mod largest_divisible_subset;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
