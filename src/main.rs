// mod zig_zag;

#[macro_use]
mod util;
pub mod best_time_to_buy_and_sell_stock_with_cooldown;

fn main() {
}

trait View {
    type Body: View;
    fn body() -> Self::Body;
}
