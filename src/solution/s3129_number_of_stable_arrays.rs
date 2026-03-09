
pub struct Solution {}

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let one = one as usize;
        let zero = zero as usize;
        let limit = limit as usize;
        let modu = 1_000_000_007;
        let mut dp = vec![vec![vec![0;2]; one+1]; zero+1];

        for i in 0..=zero.min(limit) {
            dp[i][0][0] = 1i64;
        }

        for j in 0..=one.min(limit) {
            dp[0][j][1] = 1i64;
        }

        for i in 1..=zero {
            for j in 1..=one {
                dp[i][j][0] = (dp[i-1][j][0] + dp[i-1][j][1] + modu - if i as i32 - limit as i32 - 1 >= 0 { dp[i-limit-1][j][1] } else { 0 }) % modu;
                dp[i][j][1] = (dp[i][j-1][0] + dp[i][j-1][1] + modu - if j as i32 - limit as i32 - 1 >= 0 { dp[i][j-limit-1][0] } else { 0 }) % modu;
            }
        }
        (dp[zero][one][0] as i32 + dp[zero][one][1] as i32) % modu as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(14, Solution::number_of_stable_arrays(3, 3, 2));

    }
}
