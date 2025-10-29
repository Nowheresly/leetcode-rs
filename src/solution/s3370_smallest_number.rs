pub struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut power = 0;
        loop {
            if i32::pow(2, power) > n {
                return i32::pow(2, power) - 1;
            }
            power += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::smallest_number(5));
        assert_eq!(15, Solution::smallest_number(10));
        assert_eq!(3, Solution::smallest_number(3));
    }
}
