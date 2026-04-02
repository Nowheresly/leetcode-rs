pub struct Solution {}

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        let min = i32::MIN / 2;
        let mut dp = vec![vec![vec![min;3]; n]; m];
        dp[0][0][0] = coins[0][0];
        if coins[0][0] < 0 {
            dp[0][0][1] = 0;
        }

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }

                for k in 0..3 {
                    let mut prev_max = min;
                    if i > 0 {
                        prev_max = prev_max.max(dp[i - 1][j][k]);
                    }
                    if j > 0 {
                        prev_max = prev_max.max(dp[i][j - 1][k]);
                    }

                    if prev_max != min {
                        dp[i][j][k] = dp[i][j][k].max(prev_max + coins[i][j]);
                    }

                    if k > 0 && coins[i][j] < 0 {
                        let mut prev_max_k1 = min;

                        if i > 0 {
                            prev_max_k1 = prev_max_k1.max(dp[i - 1][j][k - 1]);
                        }
                        if j > 0 {
                            prev_max_k1 = prev_max_k1.max(dp[i][j - 1][k - 1]);
                        }

                        if prev_max_k1 != min {
                            dp[i][j][k] = dp[i][j][k].max(prev_max_k1);
                        }
                    }
                }
            }
        }

        dp[m-1][n-1][2].max(dp[m-1][n-1][1].max(dp[m-1][n-1][0]))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::maximum_amount(vec![vec![0,1,-1],vec![1,-2,3],vec![2,-3,4]]));

    }
}