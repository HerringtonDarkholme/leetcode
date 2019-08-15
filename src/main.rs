// mod zig_zag;

#[macro_use]
mod util;
pub mod largest_palindrome_product;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
