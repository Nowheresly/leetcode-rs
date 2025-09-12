
pub struct Solution {}

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        for c in s.chars() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::does_alice_win(String::from("leetcoder")));
        assert_eq!(false, Solution::does_alice_win(String::from("bbcd")));

    }
}
