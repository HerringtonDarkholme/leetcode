// mod zig_zag;

#[macro_use]
mod util;
mod binary_tree_level_order_traversal;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
