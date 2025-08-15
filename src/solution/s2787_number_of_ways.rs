
pub struct Solution {}

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1; // Base case: one way to form 0
        for i in 1..=n {
            let power = i.pow(x as u32);
            if power > n {
                break;
            }
            for j in (power..=n).rev() {
                dp[j as usize] = (dp[j as usize] + dp[(j - power) as usize]) % MOD;
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::number_of_ways(10,2));
        assert_eq!(2, Solution::number_of_ways(4,1));
    }
}
