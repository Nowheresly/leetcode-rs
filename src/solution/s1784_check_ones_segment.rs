pub struct Solution {}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let ch:Vec<char> = s.chars().collect();
        for i in 1..ch.len() {
            if ch[i] == '1' && ch[i-1] == '0' {
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
        assert_eq!(false, Solution::check_ones_segment(String::from("1001")));

    }
}
