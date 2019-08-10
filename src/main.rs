// mod zig_zag;

#[macro_use]
mod util;
pub mod n_repeated_element_times;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
