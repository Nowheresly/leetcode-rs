use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut set = HashSet::new();
        for i in k..=s.len() as i32 {
            set.insert(&s[i as usize - k as usize..i as usize]);
        }
        let need = 1 << k;
        return set.len() == need
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::has_all_codes(String::from("00110110"), 2));


    }
}
