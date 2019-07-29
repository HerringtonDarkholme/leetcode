// mod zig_zag;

#[macro_use]
mod util;
mod monotone_increasing_digits;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
