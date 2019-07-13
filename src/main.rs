// mod zig_zag;

#[macro_use]
mod util;
mod palindrome_partition_ii;

pub fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
