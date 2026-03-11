
pub struct Solution {}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut n = n;
        let mut todo = n;
        let mut bit = 1;
        while todo != 0 {
            n = n ^ bit;
            bit = bit << 1;
            todo = todo >> 1;
        }
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::bitwise_complement(10));


    }
}
