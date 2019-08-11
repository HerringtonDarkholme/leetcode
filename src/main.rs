// mod zig_zag;

#[macro_use]
mod util;
pub mod top_k_frequent;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
