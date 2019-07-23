// mod zig_zag;

#[macro_use]
mod util;
mod find_frequent_tree_sum;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
