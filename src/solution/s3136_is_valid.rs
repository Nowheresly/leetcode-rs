pub struct Solution {}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let vowels = "aeiouAEIOU";
        if word.len() < 3 {
            return false;
        }
        let mut has_vowel = false;
        let mut has_conso = false;
        for c in word.chars() {
            if c.is_ascii_alphanumeric() == false {
                return false;
            }
            if c.is_ascii_alphabetic() {
                if vowels.contains(c) {
                    has_vowel = true;
                } else {
                    has_conso = true;
                }
            }
        }

        return has_conso && has_vowel;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_valid(String::from("234Adas")));
        assert_eq!(false, Solution::is_valid(String::from("b3")));
        assert_eq!(false, Solution::is_valid(String::from("a3$e")));
    }
}