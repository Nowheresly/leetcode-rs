pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for n in nums {
            sum += n;
        }
        sum % k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::min_operations(vec![3,9,7], 5));
    }


}
