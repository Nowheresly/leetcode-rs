pub struct Solution {}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
        for s in strs.iter() {
            let (zeros, ones) = s.chars().fold((0, 0), |(z, o), c| {
                if c == '0' {
                    (z + 1, o)
                } else {
                    (z, o + 1)
                }
            });
            for i in (zeros..=m as usize).rev() {
                for j in (ones..=n as usize).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                }
            }
        }
        dp[m as usize][n as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::find_max_form(
                vec![
                    String::from("10"),
                    String::from("0001"),
                    String::from("111001"),
                    String::from("1"),
                    String::from("0")
                ],
                5,
                3
            )
        );
    }
}
