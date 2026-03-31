use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut res = -1;
        let mut map = HashMap::new();
        for i in 0..s.len() {
            let ch = s.as_bytes()[i] as char;
            if map.contains_key(&ch) {
                res = res.max(i as i32 - map[&ch] - 1);
            } else {
                map.insert(
                    ch,
                    i as i32
                );
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::max_length_between_equal_characters(String::from("aa")));
    }
}
