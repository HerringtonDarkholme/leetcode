// mod zig_zag;

#[macro_use]
mod util;
mod rotate_array;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
