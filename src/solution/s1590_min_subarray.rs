use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len() as i32;
        let mut sum = 0;
        for i in 0..n {
            sum = (sum + nums[i as usize]) % p;
        }
        let target = sum % p;
        if target == 0 {
            return 0;
        }
        // modulo sum -> index
        let mut map:HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);

        let mut curr_sum = 0;
        let mut min_len: i32 = n as i32;

        for i in 0..n {
            curr_sum = (curr_sum + nums[i as usize]) % p;

            let needed = (curr_sum - target + p) % p;
            if let Some(idx) = map.get(&needed) {
                min_len = min_len.min(i - *idx);
            }
            map.insert(curr_sum, i);
        }

        if min_len == n {
            return -1;
        }
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_subarray(vec![3,1,4,2], 6));
        assert_eq!(2, Solution::min_subarray(vec![6,3,5,2], 9));

    }
}
