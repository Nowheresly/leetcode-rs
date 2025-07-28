
pub struct Solution {}

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..nums.len() {
            max |= nums[i];
        }
        let mut count = 0;
        dfs(&nums, &max, 0, 0, &mut count);
        count
    }
}

fn dfs(nums: &Vec<i32>, max: &i32, index: usize, current_or: i32, count: &mut i32) {
    if index == nums.len() {
        if current_or == *max {
            *count += 1;
        }
        return;
    }

    // Include the current number
    dfs(nums, max, index + 1, current_or | nums[index], count);

    // Exclude the current number
    dfs(nums, max, index + 1, current_or, count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2, Solution::count_max_or_subsets(vec![3,1]));
        assert_eq!(7, Solution::count_max_or_subsets(vec![2,2,2]));
        assert_eq!(6, Solution::count_max_or_subsets(vec![3,2,1,5]));
    }
}
