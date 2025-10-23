pub struct Solution {}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut s = s;
        while s.len() > 2 {
            let mut tmp = String::new();
            for i in 1..s.len() {
                let d1 = s.chars().nth(i-1).unwrap().to_digit(10).unwrap();
                let d2 = s.chars().nth(i).unwrap().to_digit(10).unwrap();
                let sum10 = (d1+d2) % 10;
                tmp.push_str(&sum10.to_string());
            }
            s = tmp;
        }
        s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::has_same_digits(String::from("3902")));
        assert_eq!(false, Solution::has_same_digits(String::from("34789")));
    }
}
