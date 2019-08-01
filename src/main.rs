// mod zig_zag;

#[macro_use]
mod util;
pub mod replace_words;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
