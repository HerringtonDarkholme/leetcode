// mod zig_zag;

#[macro_use]
mod util;
pub mod equation_possible;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
