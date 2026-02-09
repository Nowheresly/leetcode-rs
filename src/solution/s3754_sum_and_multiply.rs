pub struct Solution {}

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut transform:i64 = 0;
        let mut sum : i64 = 0;
        let mut exp = 1;
        let mut n = n;
        while n > 0 {
            let digit = (n % 10) as i64;
            sum += digit;
            if digit > 0 {
                transform += digit * exp as i64;
                exp *= 10;
            }
            n /= 10;
        }
        sum * transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12340, Solution::sum_and_multiply(10203004));
        assert_eq!(1, Solution::sum_and_multiply(1000));
    }
}
