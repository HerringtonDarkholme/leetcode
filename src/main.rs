// mod zig_zag;

#[macro_use]
mod util;
mod permutations;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
