pub struct Solution {}

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut ans = 0i64;
        let n = prices.len();
        let mut cnt = 0;
        for i in 0..n {
            let val = prices[i];
            let prev = if i == 0 { val + 1 } else { prices[i - 1] };

            if val == prev -1 {
                cnt+=1;
            } else {
                ans += cnt * (cnt + 1) / 2;
                cnt = 1;
            }
        }
        ans += cnt * (cnt + 1) / 2;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::get_descent_periods(vec![3,2,1,4]));
    }
}
