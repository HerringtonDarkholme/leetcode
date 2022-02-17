struct Bank {
    balance: Vec<i64>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Self {
            balance
        }
    }
    fn check(&self, account1: i32, account2: i32) -> Option<(usize, usize)> {
        let len = self.balance.len() as i32;
        if account1 <= 0 || account1 > len {
            return None;
        }
        if account2 <= 0 || account2 > len {
            return None;
        }
        Some((account1 as usize - 1, account2 as usize - 1))
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if let Some((a1, a2)) = self.check(account1, account2) {
            if self.balance[a1] < money {
                return false;
            }
            self.balance[a1] -= money;
            self.balance[a2] += money;
            true
        } else {
            false
        }
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let len = self.balance.len() as i32;
        if account <= 0 || account > len {
            return false;
        }
        let a = account as usize - 1;
        self.balance[a] += money;
        true
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let len = self.balance.len() as i32;
        if account <= 0 || account > len {
            return false;
        }
        let a = account as usize - 1;
        if self.balance[a] < money {
            return false;
        }
        self.balance[a] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
