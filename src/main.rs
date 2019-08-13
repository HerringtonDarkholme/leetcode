// mod zig_zag;

#[macro_use]
mod util;
pub mod find_substring_inwraparound_string;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
