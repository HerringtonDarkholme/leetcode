// mod zig_zag;

#[macro_use]
mod util;
pub mod n_queens;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
