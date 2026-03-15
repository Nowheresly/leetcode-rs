struct Fancy {
    val: Vec<i64>,
    a: i64,
    b: i64,
}

const MOD: i64 = 1_000_000_007;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Self {
            val: Vec::new(),
            a: 1,
            b: 0,
        }
    }

    fn append(&mut self, val: i32) {
        let x = (val as i64 - self.b + MOD) % MOD;
        self.val.push((x * mod_pow(self.a, MOD - 2, MOD)) % MOD);
    }

    fn add_all(&mut self, inc: i32) {
        self.b = (self.b + inc as i64) % MOD;
    }

    fn mult_all(&mut self, m: i32) {
        self.a = (self.a * m as i64) % MOD;
        self.b = (self.b * m as i64) % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        if idx >= self.val.len() as i32 {
            return -1;
        }
        ((self.a * self.val[idx as usize] + self.b) % MOD) as i32
    }
}

fn mod_pow(x: i64, y: i64, modu: i64) -> i64 {
    let mut res = 1;
    let mut x = x % modu;
    let mut y = y;
    while y > 0 {
        if y & 1 == 1 {
            // y is odd
            res = (res * x) % modu;
        }
        y = y / 2;
        x = (x * x) % modu;
    }
    res
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = Fancy::new();
        obj.append(2);
        obj.add_all(3);
        obj.append(7);
        obj.mult_all(2);
        assert_eq!(10, obj.get_index(0));
    }
}
