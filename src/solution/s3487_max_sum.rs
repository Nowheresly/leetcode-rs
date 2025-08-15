pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let max = nums.iter().max().unwrap().clone();
        for num in nums {
            if num > 0 {
                set.insert(num);
            }
        }
        if set.is_empty() {
            return max;
        }
        set.iter().sum::<i32>()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::max_sum(vec![2, 5]));
        assert_eq!(1, Solution::max_sum(vec![1,1,0,1,1]));
        assert_eq!(3, Solution::max_sum(vec![1,2,-1,-2,1,0,-1]));
        assert_eq!(
            19,
            Solution::max_sum(vec![1, 8, 10])
        );
    }
}
