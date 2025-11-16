pub struct Solution {}

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let modus = 1000000007;
        let mut ans:i64 = 0;

        let mut count = 0;
        for c in s.chars() {
            if c == '1' {
                count += 1;
                continue;
            }
            ans += (count+1) * count / 2;
            ans %= modus;
            count = 0;
        }
        ans += (count+1) * count / 2;
        ans %= modus;
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::num_sub(String::from("0110111")));
        assert_eq!(2, Solution::num_sub(String::from("101")));
        assert_eq!(21, Solution::num_sub(String::from("111111")));
    }
}
