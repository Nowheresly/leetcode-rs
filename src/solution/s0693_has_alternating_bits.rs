pub struct Solution {}

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut max = 1i64;
        let mut d = 1;
        for i in 1..32 {
            max *= 2;
            if max >= n as i64 {
                d = i;
                break;
            }
        }
        for i in 1..d {
            let prev = n & ( 1 << (i-1));
            let cur = n & (1 << i);
            if prev == 0 && cur == 0 {
                return false;
            }
            if prev != 0 && cur != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::has_alternating_bits(5));
        assert_eq!(false, Solution::has_alternating_bits(7));
        assert_eq!(false, Solution::has_alternating_bits(11));

    }
}
