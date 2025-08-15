pub struct Solution {}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        let hex = "0123456789abcdef".chars().collect::<Vec<_>>();
        let mut n = num as u32;
        let mut s = String::new();
        while n != 0 {
            let r = n & 15;
            s.push(hex[r as usize]);
            n = n >> 4;
        }
        s.chars().rev().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("1a"), Solution::to_hex(26));
        assert_eq!(String::from("ffffffff"), Solution::to_hex(-1));

    }
}
