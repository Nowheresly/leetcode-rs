pub struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut low = low;
        if low % 2 == 0 {
            low += 1;
        }
        if low > high { 0} else { (high - low) / 2 + 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_odds(3, 7));

    }
}
