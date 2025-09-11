use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut vowels_from_s = vec![];
        for c in s.chars() {
            let vowel = c.to_lowercase().next().unwrap();
            if vowels.contains(&vowel) {
                vowels_from_s.push(c);
            }
        }
        vowels_from_s.sort_by(|a, b| {
            if a.is_uppercase() && b.is_uppercase() {
                return a.cmp(b);
            }
            if a.is_lowercase() && b.is_lowercase() {
                return a.cmp(b);
            }
            if a.is_uppercase() && b.is_lowercase() {
                return Ordering::Less;
            }
            Ordering::Greater
        });
        let mut result = String::new();
        let mut j = 0;
        for c in s.chars() {
            let vowel = c.to_lowercase().next().unwrap();
            if vowels.contains(&vowel) {
                result.push(vowels_from_s[j]);
                j += 1;
            } else {
                result.push(c);
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
        assert_eq!(String::from("lEOtcede"), Solution::sort_vowels(String::from("lEetcOde")));
        assert_eq!(String::from("lYmpH"), Solution::sort_vowels(String::from("lYmpH")));

    }
}
