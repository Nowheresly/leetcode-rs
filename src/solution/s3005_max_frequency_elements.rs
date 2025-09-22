
pub struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 101];
        let mut max = 0;
        for i in 0..nums.len() {
            freq[nums[i] as usize] += 1;
            max = max.max(freq[nums[i] as usize]);
        }
        let mut ret = 0;
        for i in 0..101 {
            if freq[i] == max {
                ret += max;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_frequency_elements(vec![1,2,2,3,1,4]));

    }
}
