pub struct Solution {}

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut prefix = vec![0; n+1];
        for i in 0..n {
            prefix[i+1] = prefix[i] + nums[i] as i64;
        }
        let mut max = i64::MIN;
        for i in 0..k as usize {
            let mut cur_sum:i64 = 0;
            for j in ((i+k as usize)..=n).step_by(k as usize) {
                cur_sum = 0.max(cur_sum);
                let sum = prefix[j] - prefix[j - k as usize];
                cur_sum += sum;
                max = max.max(cur_sum);
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        //assert_eq!(3, Solution::max_subarray_sum(vec![1,2], 2));
        //assert_eq!(-10, Solution::max_subarray_sum(vec![-1,-2,-3,-4,-5], 4));
        assert_eq!(4, Solution::max_subarray_sum(vec![-5,1,2,-3,4], 2));

    }
}
