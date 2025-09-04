pub struct Solution {}

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        if i32::abs(z - x) < i32::abs(z - y) {
            return 1;
        }
        if i32::abs(z - x) > i32::abs(z - y) {
            return 2;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::find_closest(2, 7, 4));
        assert_eq!(2, Solution::find_closest(2, 5, 6));
        assert_eq!(0, Solution::find_closest(1, 5, 3));
    }
}
