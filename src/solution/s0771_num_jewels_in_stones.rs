pub struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        for c in jewels.chars() {
            set.insert(c);
        }
        let mut count = 0;
        for c in stones.chars() {
            if set.contains(&c) {
                count += 1;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")));
        assert_eq!(0, Solution::num_jewels_in_stones(String::from("z"), String::from("ZZ")));
    }
}
