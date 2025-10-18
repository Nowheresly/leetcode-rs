pub struct Solution {}

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut res = 1;
        let mut prev = nums[0];
        let mut curk = -k + 1;
        for i in 1..n {
            let val = nums[i];
            if val == prev {
                prev = val;
                if curk <= k {
                    curk += 1;
                    res += 1;
                    continue;
                }
                continue;
            }
            // val != prev
            let last_k_applied = curk - 1;
            let last_val_saved = prev + last_k_applied;
            let cur_val_to_save = (val - k).max(last_val_saved + 1);
            let delta_applied = cur_val_to_save - val;
            curk = delta_applied + 1;
            res += 1;
            prev = val;
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::max_distinct_elements(vec![1,2,2,3,3,4], 2));
        assert_eq!(3, Solution::max_distinct_elements(vec![4,4,4,4], 1));
    }
}