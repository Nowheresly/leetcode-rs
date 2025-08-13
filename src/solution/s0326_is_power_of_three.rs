pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let max_power_of_3: i32 = 3i32.pow(19);
        return n > 0 && max_power_of_3 % n == 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_power_of_three(27));
        assert_eq!(false, Solution::is_power_of_three(0));
        assert_eq!(false, Solution::is_power_of_three(-1));
    }
}
