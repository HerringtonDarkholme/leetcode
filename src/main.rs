// mod zig_zag;

#[macro_use]
mod util;
mod regular_expression_matching;

pub fn main() {
    regular_expression_matching::Solution::is_match("test".to_owned(), "test".to_owned());
}
