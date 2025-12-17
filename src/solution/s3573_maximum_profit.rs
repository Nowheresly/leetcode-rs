pub struct Solution {}

pub enum State {
    NOPE,
    BUY,
    SHORT,
}
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let mut memo = vec![vec![vec![-1i64; 3]; (k + 1) as usize]; prices.len()];

        dfs(&prices, k as usize, &mut memo, 0 as usize, State::NOPE)
    }
}

fn dfs(
    prices: &Vec<i32>,
    k: usize,
    memo: &mut Vec<Vec<Vec<i64>>>,
    day: usize,
    state: State,
) -> i64 {
    if k == 0 {
        return 0;
    }
    if day >= prices.len() {
        return 0;
    }
    let st = match state {
        State::NOPE => 0,
        State::BUY => 1,
        State::SHORT => 2,
    };
    if memo[day][k][st] != -1 {
        return memo[day][k][st];
    }
    match state {
        State::NOPE => {
            // we can buy, short, nope
            // case buy:
            let gain_buy = -1i64 * prices[day] as i64 + dfs(prices, k, memo, day + 1, State::BUY);
            let mut gain_short = prices[day] as i64 + dfs(prices, k, memo, day + 1, State::SHORT);
            if day == prices.len() - 1 {
                gain_short = 0;
            }
            let gain_nope = dfs(prices, k, memo, day + 1, State::NOPE);
            let ret = gain_buy.max(gain_short).max(gain_nope);
            memo[day][k][st] = ret;
            ret
        }
        State::BUY => {
            // we can sell or nope
            let gain_sell = prices[day] as i64 + dfs(prices, k - 1, memo, day + 1, State::NOPE);
            let gain_nope = dfs(prices, k, memo, day + 1, State::BUY);
            let ret = gain_sell.max(gain_nope);
            memo[day][k][st] = ret;
            ret
        }
        State::SHORT => {
            // we can buy or nope
            let gain_buy =
                -1i64 * prices[day] as i64 + dfs(prices, k - 1, memo, day + 1, State::NOPE);
            let gain_nope;
            if day == prices.len() - 1 {
                gain_nope = gain_buy;
            } else {
                gain_nope = dfs(prices, k, memo, day + 1, State::SHORT);
            }
            let ret = gain_nope.max(gain_buy);
            memo[day][k][st] = ret;
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(14, Solution::maximum_profit(vec![1, 7, 9, 8, 2], 2));
        assert_eq!(
            36,
            Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3)
        );
    }
}
