
pub struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut sum = 1;
        for i in 2..num {
            if num % i == 0 {
                sum += i;
            }
            if sum > num {
                return false;
            }
        }
        sum == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::check_perfect_number(28));

    }
}
