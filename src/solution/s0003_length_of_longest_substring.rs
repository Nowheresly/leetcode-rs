
pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut map = std::collections::HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        for right in 0..chars.len() {
            let c = chars[right];
            let count = map.entry(c).or_insert(0);
            *count += 1;
            while *map.get(&c).unwrap() > 1 {
                let left_c = chars[left];
                let left_count = map.get_mut(&left_c).unwrap();
                *left_count -= 1;
                left += 1;
            }
            res = res.max((right - left + 1) as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
        assert_eq!(1, Solution::length_of_longest_substring(String::from("bbbbb")));
        assert_eq!(3, Solution::length_of_longest_substring(String::from("pwwkew")));


    }
}
