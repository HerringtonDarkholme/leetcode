impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let (mut time, mut wait) = (1, 0);
        let n = customers.len() as f64;
        for customer in customers {
            let (arrival, cook) = (customer[0] as i64, customer[1] as i64);
            time = time.max(arrival) + cook;
            wait += time - arrival;
        }
        wait as f64 / n
    }
}
