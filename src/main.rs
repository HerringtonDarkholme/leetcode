// mod zig_zag;

#[macro_use]
mod util;
pub mod network_delay_time;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
