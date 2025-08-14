
pub struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut prev = None;
        let mut count = 0;
        let mut result = String::new();
        for c in num.chars() {
            if prev.is_none() {
                prev = Some(c);
                count = 1;
                continue;
            }
            if Some(c) == prev {
                count += 1;
                if count >= 3 {
                    result = result.max(format!("{}{}{}", c, c, c));
                }
            } else {
                prev = Some(c);
                count = 1;
            }
        }
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(String::from("777"), Solution::largest_good_integer(String::from("6777133339")));
        assert_eq!(String::from("000"), Solution::largest_good_integer(String::from("2300019")));
        assert_eq!(String::from(""), Solution::largest_good_integer(String::from("42352338")));


    }
}
