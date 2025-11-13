
pub struct Solution {}

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut count_ones = 0;
        let mut ans = 0;
        let mut i = 0;
        let chars: Vec<char> = s.chars().collect();

        while i < s.len() {
            if chars[i] == '0' {
                while i + 1  < s.len() && chars[i + 1] == '0' {
                    i += 1;
                }
                ans += count_ones;
            } else {
                count_ones += 1;
            }
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_operations(String::from("1001101")));


    }
}
