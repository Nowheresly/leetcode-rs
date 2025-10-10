pub struct Solution {}

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let mut max = std::i32::MIN;
        let mut dp = vec![0; n];
        let k = k as usize;

        for i in (0..n).rev() {
            dp[i] = energy[i];
            if i+k < n {
                dp[i] += dp[i+k];
            }
            max = std::cmp::max(max, dp[i]);
        }
        max
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::maximum_energy(vec![5,2,-10,-5,1], 3));
        assert_eq!(-1, Solution::maximum_energy(vec![-2,-3,-1], 2));

    }
}