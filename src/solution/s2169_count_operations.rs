pub struct Solution {}

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        if  num2 > num1 {
            return Self::count_operations(num2, num1);
        }
        let mut res = 0;
        let mut num1 = num1;
        let mut num2 = num2;
        while num1 != 0 && num2 != 0 {
            res += num1 / num2;
            let temp = num2;
            let remainder = num1 % num2;
            num1 = temp;
            num2 = remainder;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_operations(2, 3));
        assert_eq!(1, Solution::count_operations(10, 10));
    }
}
