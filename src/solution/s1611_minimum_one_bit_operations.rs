
pub struct Solution {}

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {

        for k in (0..32).rev() {
            let val = n & (1 << k);
            if (val | 0) == (1 << k) {
                return ((1 << (k + 1)) - 1) ^ Solution::minimum_one_bit_operations(n ^ val);
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::minimum_one_bit_operations(3));
        assert_eq!(4, Solution::minimum_one_bit_operations(6));

    }
}
