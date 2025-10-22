use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        nums.sort();
        let mut res = 0;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut l = 0;
        let mut r = 0;
        // target is in the array
        for x in nums.iter() {
            while r < n && nums[r] <= x + k {
                map.insert(nums[r] as i32, map.get(&nums[r]).unwrap_or(&0) + 1);
                r += 1;
            }
            while l < n && nums[l] < x - k {
                map.insert(nums[l] as i32, map.get(&nums[l]).unwrap_or(&0) - 1);
                l += 1;
            }
            let max_ops = r as i32 - l as i32 - map.get(x).unwrap_or(&0);
            res = res.max( max_ops.min(num_operations as i32) + map.get(x).unwrap_or(&0) );
        }
        // target is not in the array
        l = 0;
        for r in 0..n {
            while l < n && nums[l] + (2*k) < nums[r] {
                l += 1;
            }
            let max_ops = (r - l + 1) as i32;
            res = res.max( max_ops.min(num_operations as i32) );
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::max_frequency(vec![1, 4, 5], 1, 2));
        assert_eq!(2, Solution::max_frequency(vec![5,11,20,20], 5, 1));
    }
}
