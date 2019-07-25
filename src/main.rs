// mod zig_zag;

#[macro_use]
mod util;
mod is_rational_equal;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
