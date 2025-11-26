pub struct Solution {}

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![vec![0; k as usize]; n]; m];
        for i in 0..k {
            dp[0][0][i as usize] = if grid[0][0] % k == i { 1 } else { 0 };
        }

        for i in 0..m {
            for j in 0..n {
                let val = grid[i][j] % k;
                for g in 0..k as usize {
                    if i > 0 && dp[i - 1][j][g] > 0 {
                        dp[i][j][(val as usize + g) % k as usize] += dp[i - 1][j][g];
                        dp[i][j][(val as usize + g) % k as usize] =
                            dp[i][j][(val as usize + g) % k as usize] % MOD;
                    }
                    if j > 0 && dp[i][j - 1][g] > 0 {
                        dp[i][j][(val as usize + g) % k as usize] += dp[i][j - 1][g];
                        dp[i][j][(val as usize + g) % k as usize] =
                            dp[i][j][(val as usize + g) % k as usize] % MOD;
                    }
                }
            }
        }

        dp[m-1][n-1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3)
        );
    }
}
