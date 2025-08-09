pub struct Solution {}

impl Solution {
    pub fn power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        (n & (-n)) == n
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::power_of_two(1));
        assert_eq!(true, Solution::power_of_two(16));
        assert_eq!(false, Solution::power_of_two(3));
    }
}
