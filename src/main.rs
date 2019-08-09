// mod zig_zag;

#[macro_use]
mod util;
pub mod first_missing_positive;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
