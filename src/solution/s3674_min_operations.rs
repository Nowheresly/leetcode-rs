pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let val = nums.get(0).unwrap();
        for i in nums.iter() {
            if *i != *val {
                return 1;
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
        assert_eq!(1, Solution::min_operations(vec![1,2]));
        assert_eq!(0, Solution::min_operations(vec![5,5,5]));


    }
}
