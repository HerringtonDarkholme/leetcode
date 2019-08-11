// mod zig_zag;

#[macro_use]
mod util;
pub mod word_ladder_ii;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
