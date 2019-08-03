// mod zig_zag;

#[macro_use]
mod util;
pub mod course_schedule;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
