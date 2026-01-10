pub struct Solution {}

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let ch1 = s1.chars().collect::<Vec<char>>();
        let ch2 = s2.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![0; ch2.len()+1]; ch1.len()+1];

        for i in (0..ch2.len()).rev() {
            dp[ch1.len()][i] = dp[ch1.len()][i+1] + ch2[i] as i32;
        }

        for i in (0..ch1.len()).rev() {
            dp[i][ch2.len()] = dp[i+1][ch2.len()] + ch1[i] as i32;
        }

        for i in (0..ch1.len()).rev() {
            let c1 = ch1[i];
            for j in (0..ch2.len()).rev() {
                let c2 = ch2[j];
                if c1 == c2 {
                    dp[i][j] = dp[i+1][j+1];
                    continue;
                }
                let cost_case_1 = c1 as i32 + dp[i+1][j];
                let cost_case_2 = c2 as i32 + dp[i][j+1];
                let cost_case_3 = c1 as i32 + c2 as i32 + dp[i+1][j+1];
                dp[i][j] = cost_case_1.min(cost_case_2.min(cost_case_3));
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
        assert_eq!(403, Solution::minimum_delete_sum(String::from("delete"), String::from("leet")));
    }
}
