pub struct Solution {}

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        for k in 1..33 {
            let curr:i64 = num1 as i64 - k * num2 as i64;
            if k >= i64::count_ones(curr) as i64 && curr >= k {
                return k as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::make_the_integer_zero(3,  -2));
        assert_eq!(-1, Solution::make_the_integer_zero(5,  7));
    }
}
