// mod zig_zag;

#[macro_use]
mod util;
mod binary_tree_maximum_path_sum;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
