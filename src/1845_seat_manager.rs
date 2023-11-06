use std::collections::BinaryHeap;
struct SeatManager {
    seats: BinaryHeap<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            seats: (1..=n).map(|i| -i).collect()
        }
    }
    
    fn reserve(&mut self) -> i32 {
        -self.seats.pop().unwrap()
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(-seat_number)
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
