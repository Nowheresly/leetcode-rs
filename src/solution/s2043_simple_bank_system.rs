
struct Bank {
    balance: Vec<i64>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 == 0 || account2 == 0 || account1 > self.balance.len() as i32 || account2 > self.balance.len() as i32 {
            return false;
        }
        if self.balance[account1 as usize - 1] < money {
            return false;
        }
        self.balance[account1 as usize - 1] -= money;
        self.balance[account2 as usize - 1] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account == 0 || account > self.balance.len() as i32 {
            return false;
        }
        self.balance[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account == 0 || account > self.balance.len() as i32 {
            return false;
        }
        if self.balance[account as usize - 1] < money {
            return false;
        }
        self.balance[account as usize - 1] -= money;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let balance = vec![10, 100, 20, 50, 30];
        let account1 = 3;
        let account2 = 4;
        let account = 5;
        let money = 10;
        let mut obj = Bank::new(balance);
        assert_eq!(true, obj.transfer(account1, account2, money));
        assert_eq!(true, obj.deposit(account, money));
        assert_eq!(true, obj.withdraw(account, money));
    }
}
