pub struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![1000001; n]; m];
        dp[m-1][n-1] = 0;
        let mut max = grid[0][0];
        for i in 0..m {
            for j in 0..n {
                max = max.max(grid[i][j]);
            }
        }
        let mut tcost = vec![1000001; max as usize + 1];
        for t in 0..=k {
            for i in (0..m).rev() {
                for j in (0..n).rev() {
                    if i + 1 < m {
                        dp[i][j] = dp[i][j].min(dp[i + 1][j] + grid[i + 1][j]);
                    }
                    if j + 1 < n {
                        dp[i][j] = dp[i][j].min(dp[i][j + 1] + grid[i][j + 1]);
                    }
                    if t > 0 {
                        dp[i][j] = dp[i][j].min(tcost[grid[i][j] as usize]);
                    }
                }
            }
            // compute tcost
            for i in 0..m {
                for j in 0..n {
                    tcost[grid[i][j] as usize] = dp[i][j].min(tcost[grid[i][j] as usize]);
                }
            }
            for i in 1..tcost.len() {
                tcost[i] = tcost[i].min(tcost[i - 1]);
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            7,
            Solution::min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2)
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            9,
            Solution::min_cost(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1)
        );
    }
}
