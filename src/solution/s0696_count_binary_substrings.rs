pub struct Solution {}

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut res = 0;
        let ch: Vec<char> = s.chars().collect();
        let mut prev = 0;
        let mut cur = 1;
        for i in 1..ch.len() {
            if ch[i - 1] != ch[i] {
                res += cur.min(prev);
                prev = cur;
                cur = 1;
            } else {
                cur += 1;
            }
        }
        res + prev.min(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::count_binary_substrings(String::from("00110011"))
        );
    }
}
