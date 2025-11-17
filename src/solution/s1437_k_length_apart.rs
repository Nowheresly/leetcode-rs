pub struct Solution {}

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut i = 0;
        while i < n && nums[i] == 0 {
            i += 1;
        }
        i += 1;
        let mut cnt = 0;
        while i < n {
            let val = nums[i];
            if val == 0 {
                cnt += 1;
            } else {
                if cnt < k as usize {
                    return false;
                }
                cnt = 0;
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2)
        );
    }
}
