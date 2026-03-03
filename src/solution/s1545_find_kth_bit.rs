pub struct Solution {}

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }
        let len = 1 << n;
        if k < len / 2 {
            return Solution::find_kth_bit(n - 1, k);
        }
        if k == len / 2 {
            return '1';
        }
        let res = Solution::find_kth_bit(n - 1, len - k);
        if res == '0' {
            return '1';
        }
        '0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!('0', Solution::find_kth_bit(3, 1));

    }
}
