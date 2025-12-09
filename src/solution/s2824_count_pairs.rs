
pub struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i]+nums[j] < target {
                    ans += 1;
                }
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
        assert_eq!(3, Solution::count_pairs(vec![-1,1,2,3,1], 2));

    }
}
