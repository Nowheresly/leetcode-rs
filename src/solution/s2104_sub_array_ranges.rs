pub struct Solution {}

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        for i in 0..nums.len() {
            let mut min = nums[i];
            let mut max = nums[i];
            for j in i + 1..nums.len() {
                min = min.min(nums[j]);
                max = max.max(nums[j]);
                ans += (max - min) as i64;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::sub_array_ranges(vec![1, 2, 3]));
        assert_eq!(4, Solution::sub_array_ranges(vec![1, 3, 3]));
    }
}
