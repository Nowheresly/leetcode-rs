
pub struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut sb:Vec<char> = vec![];
        let mut prev:char = s.chars().nth(0).unwrap();
        let mut cnt = 1;
        sb.push(prev);
        for c in s.chars().skip(1) {
            if c == prev {
                cnt += 1;
                if cnt < 3 {
                    sb.push(c);
                }
            } else {
                sb.push(c);
                prev = c;
                cnt = 1;
            }
        }
        sb.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!("leetcode".to_string(), Solution::make_fancy_string("leeetcode".to_string()));
    }
}
