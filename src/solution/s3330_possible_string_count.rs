pub struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut ret:i32 = 1;
        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                continue;
            }
            if c == word.chars().nth(i-1).unwrap() {
                ret = ret + 1;
            }
        }
        
        return ret;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::possible_string_count("abbcccc".to_string()));
        assert_eq!(1, Solution::possible_string_count("abcd".to_string()));
        assert_eq!(4, Solution::possible_string_count("aaaa".to_string()));
    }
}