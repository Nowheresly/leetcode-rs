use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted: Vec<i32> = nums.clone();
        sorted.sort();
        let mut i = 0;
        let mut j:i32 = (nums.len() - 1) as i32;

        while i <= j {
            if nums[i as usize] != sorted[i as usize] && nums[j as usize] != sorted[j as usize] {
                break;
            }
            if nums[i as usize] == sorted[i as usize] {
                i += 1;
            }
            if nums[j as usize] == sorted[j as usize] {
                j -= 1;
            }
        }
        if i >j {
            return 0;
        }
        j -i+1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::find_unsorted_subarray(vec![2,6,4,8,10,9,15]));
        assert_eq!(0, Solution::find_unsorted_subarray(vec![1, 2, 3, 4]));
        assert_eq!(0, Solution::find_unsorted_subarray(vec![1]));
    }
}
