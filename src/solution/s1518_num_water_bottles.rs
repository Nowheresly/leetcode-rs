pub struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut full = num_bottles;
        let mut empty = 0;
        let mut res = 0;

        while full > 0 {
            res += full;
            empty += full;
            full = empty / num_exchange;
            empty -= full * num_exchange;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(13, Solution::num_water_bottles(9, 3));
        assert_eq!(19, Solution::num_water_bottles(15, 4));
    }
}
