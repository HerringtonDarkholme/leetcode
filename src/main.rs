// mod zig_zag;

#[macro_use]
mod util;
pub mod parse_bool_expr;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
