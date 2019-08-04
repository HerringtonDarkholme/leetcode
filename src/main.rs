// mod zig_zag;

#[macro_use]
mod util;
pub mod min_height_trees;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
