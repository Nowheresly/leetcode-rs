pub struct Solution {}

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let ch:Vec<char> = s.chars().collect();
        let mut res = 0;
        let mut carry = 0;
        for i in (1..ch.len()).rev() {
            let cur = (ch[i] as i32 - '0' as i32) + carry;
            if cur == 1 {
                res += 2;
                carry = 1;
            } else {
                res += 1;
            }
        }
        res + carry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::num_steps(String::from("1101")));


    }
}
