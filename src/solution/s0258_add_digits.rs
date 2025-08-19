pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        loop {
            let mut ret = 0;
            while num > 0 {
                ret += num % 10;
                num /= 10;
            }
            if ret < 10 {
                return ret;
            }
            num = ret;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::add_digits(38));
        assert_eq!(0, Solution::add_digits(0));

    }
}
