

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k = k as usize;

        let mut prefix = vec![0i64; n+1];
        let mut prefix2 = vec![0i64; n+1];
        for i in 0..n {
            prefix[i+1] = prefix[i] + (prices[i] as i64 * strategy[i] as i64);
            prefix2[i+1] = prefix2[i] + prices[i] as i64;
        }
        let mut ans = prefix[n];
        for i in 0..=(n-k) {
            let mut profit = prefix[i];
            profit += prefix[n] - prefix[i+k];
            profit += prefix2[i+k] - prefix2[i +k / 2];
            ans = ans.max(profit);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            10,
            Solution::max_profit(vec![4,2,8], vec![-1,0,1], 2)
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            9,
            Solution::max_profit(vec![5,4,3], vec![1,1,0], 2)
        );
    }
}
