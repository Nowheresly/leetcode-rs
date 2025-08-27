
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        for i in 0..str.len()/2 {
            let a = str.chars().nth(i).unwrap();
            let b = str.chars().nth(str.len() - i-1).unwrap();
            if a != b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));


    }
}
