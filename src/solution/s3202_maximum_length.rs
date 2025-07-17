pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 2;
        for j in 0..k {
            let mut dp = vec![0; k as usize];
            for i in 0..nums.len() {
                let modu = nums[i] % k;
                let posi = (j - modu + k) % k;
                dp[modu as usize] = dp[posi as usize] + 1;
            }
            for i in 0..k {
                res = res.max(dp[i as usize]);
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::maximum_length(vec![1,2,3,4,5], 2));
        assert_eq!(4, Solution::maximum_length(vec![1,4,2,3,1,4], 3));

    }
}