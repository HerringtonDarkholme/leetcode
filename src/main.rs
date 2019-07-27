// mod zig_zag;

#[macro_use]
mod util;
mod min_swaps_couples;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
