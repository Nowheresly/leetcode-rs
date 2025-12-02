pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut u0 = 1.0f64;
        let mut prev = u0 + 2.0f64;
        while f64::abs(u0 - prev) >=  1.0 {
            prev = u0;
            u0 = 1.0/2.0 * (u0 + x as f64 / u0);
        }
        u0 as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));

    }
}
