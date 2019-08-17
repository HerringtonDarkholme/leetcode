// mod zig_zag;

#[macro_use]
mod util;
pub mod project_area_of_3d_shapes;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
