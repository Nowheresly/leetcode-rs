pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = 0;
        let mut left = 0;
        let mut deleted = nums[left] == 0;
        for right in (left +1)..n {
            if nums[right] == 1 {
                continue;
            }
            if deleted {
                max = max.max(right - left - 1);
                if nums[left] == 0 {
                    left += 1;
                } else {
                    while nums[left] == 1 {
                        left += 1;
                    }
                    left += 1;
                }
                continue;
            }
            deleted = true;
        }
        max = max.max(n - left - 1);
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::longest_subarray(vec![1,1,0,1])
        );
        assert_eq!(
            5,
            Solution::longest_subarray(vec![0,1,1,1,0,1,1,0,1])
        );
        assert_eq!(
            2,
            Solution::longest_subarray(vec![1,1,1])
        );
    }
}
