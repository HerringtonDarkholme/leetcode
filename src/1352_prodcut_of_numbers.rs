struct ProductOfNumbers {
    list: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        ProductOfNumbers {
            list: vec![1],
        }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.list = vec![1];
            return
        }
        let n = num * self.list.last().unwrap();
        self.list.push(n);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k > self.list.len() - 1 {
            0
        } else {
            let l = self.list.len() - 1;
            self.list[l] / self.list[l - k]
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
