pub struct Solution {}

const MOD:i64 = 1000000007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut x:i64 = 6;
        let mut y:i64 = 6;
        for _ in 2..=n {
            let new_x = ( 3 * x + 2 * y ) % MOD;
            let new_y = ( 2 * x + 2 * y ) % MOD;
            x = new_x;
            y = new_y;
        }
        ((x + y) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::num_of_ways(1));
        assert_eq!(30228214, Solution::num_of_ways(5000));

    }
}
