// mod zig_zag;

#[macro_use]
mod util;
mod matchsticks_to_sqaure;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
