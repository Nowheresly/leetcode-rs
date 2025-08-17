pub struct Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let ch1 = num1.chars().rev().collect::<Vec<char>>();
        let ch2 = num2.chars().rev().collect::<Vec<char>>();
        let mut res = Vec::new();
        let mut carry = 0;
        for i in 0..ch1.len().max(ch2.len()) {
            let n1 = ch1.get(i).unwrap_or(&'0');
            let n2 = ch2.get(i).unwrap_or(&'0');
            let sum = n1.to_digit(10).unwrap() + n2.to_digit(10).unwrap() + carry;
            res.push(sum % 10);
            carry = sum / 10;
        }
        if carry > 0 {
            res.push(carry);
        }
        res.iter().rev().map(|x| x.to_string()).collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("134"), Solution::add_strings(String::from("11"), String::from("123")));
        assert_eq!(String::from("533"), Solution::add_strings(String::from("456"), String::from("77")));
        assert_eq!(String::from("0"), Solution::add_strings(String::from("0"), String::from("0")));
    }
}
