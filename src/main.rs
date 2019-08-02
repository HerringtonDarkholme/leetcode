// mod zig_zag;

#[macro_use]
mod util;
pub mod reverse_string_ii;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
