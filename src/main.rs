// mod zig_zag;

#[macro_use]
mod util;
pub mod find_disappeared_numbers;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
