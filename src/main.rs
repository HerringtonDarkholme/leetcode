// mod zig_zag;

#[macro_use]
mod util;
pub mod number_of_atoms;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
