
pub struct Solution {}

impl Solution {
    pub fn max_a(n: i32) -> i32 {
        let mut dp = vec![0i32; n as usize + 1];
        for i in 0..=n {
            dp[i as usize] = i;
        }
        for i in 0..=(n-3) {
            for j in (i+3)..=n.min(i+6) {
                dp[j as usize] = dp[j as usize].max((j -i -1) * dp[i as usize])
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

        assert_eq!(3, Solution::max_a(3));
        assert_eq!(9, Solution::max_a(7));

    }
}
