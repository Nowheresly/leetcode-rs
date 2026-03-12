
pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {

        match word.find(ch) {
            Some(idx) => {

                let split_at = idx + ch.len_utf8();

                let mut result: String = word[..split_at].chars().rev().collect();

                result.push_str(&word[split_at..]);

                result
            }
            None => word,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(String::from("dcbaefd"), Solution::reverse_prefix(String::from("abcdefd"), 'd'));

    }
}
