
pub struct Solution {}

impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for num in nums.iter() {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut max = -1;
        for num in nums.iter() {
            if map[&num] == 1 {
                max = max.max(*num);
            }
        }
        max
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::largest_unique_number(vec![5,7,3,9,4,9,8,3,1]));
        assert_eq!(-1, Solution::largest_unique_number(vec![8,8,9,9]));
    }
}
