pub struct Solution {}

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0; n+1];
        for i in 0..n {
            prefix[i+1] = prefix[i] + nums[i];
        }
        let mut res = i32::MAX;
        for i in l..=r {
            for j in 0..=(n-i as usize) {
                let sum = prefix[j+i as usize] - prefix[j];
                if sum <= 0 {
                    continue;
                }
                res = res.min(sum);
            }
        }
        if res == i32::MAX {
            return -1;
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3));
    }
}