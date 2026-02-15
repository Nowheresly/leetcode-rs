pub struct Solution {}

impl Solution {
    pub fn valid_word_square(words: Vec<String>) -> bool {
        let n = words.len();
        for row in 0..n {
            let word = &words[row];
            let mut sb = String::new();
            for i in 0..n {
                let w = &words[i];
                if row < w.len() {
                    sb.push(w.chars().nth(row).unwrap());
                }
            }
            if word != &sb {
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
        assert_eq!(
            true,
            Solution::valid_word_square(vec![
                String::from("abcd"),
                String::from("bnrt"),
                String::from("crmy"),
                String::from("dtye")
            ])
        );

        assert_eq!(
            true,
            Solution::valid_word_square(vec![
                String::from("abcd"),
                String::from("bnrt"),
                String::from("crm"),
                String::from("dt")
            ])
        );
    }
}
