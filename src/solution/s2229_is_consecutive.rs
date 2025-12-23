
pub struct Solution {}

impl Solution {
    pub fn is_consecutive(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let n = nums.len() as i32;
        let mut min = nums[0];
        let mut count0= 0;
        for i in 0..n as usize {
            min = min.min(nums[i]);
            if nums[i] == 0 {
                count0 += 1;
            }
        }
        if count0 > 1 {
            return false;
        }
        for i in 0..n as usize {
            let val = nums[i].abs();
            if val >= min + n {
                return false;
            }
            nums[val as usize - min as usize] = -1 * nums[val as usize - min as usize].abs();
        }
        for i in 0..n as usize {
            if nums[i] > 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_consecutive(vec![1,3,4,2]));

    }
}
