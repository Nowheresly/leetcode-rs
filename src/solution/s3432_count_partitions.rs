pub struct Solution {}

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for num in nums.iter() {
            total += num;
        }
        if total % 2 != 0 {
            return 0;
        }
        nums.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::count_partitions(vec![10,10,3,7,6]));
    }
}