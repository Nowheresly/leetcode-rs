
pub struct Solution {}

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let (xeven, xodd) = if n % 2 == 0 {
            (n as i64 / 2, n as i64 / 2)
        } else {
            ((n - 1) as i64 / 2, (n+1) as i64 / 2)
        };
        let (yeven, yodd) = if m % 2 == 0 {
            (m as i64 / 2, m as i64 / 2)

        } else {
            ((m - 1) as i64 / 2, (m+1) as i64 / 2)
        };

        xeven * yodd + xodd * yeven
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::flower_game(3, 2));
        assert_eq!(0, Solution::flower_game(1, 1));
    }
}
