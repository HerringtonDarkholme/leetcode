// mod zig_zag;

#[macro_use]
mod util;
pub mod build_tree;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
