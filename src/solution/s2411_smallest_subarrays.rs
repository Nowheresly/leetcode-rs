pub struct Solution {}

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut last_seen:Vec<i32> = vec![-1; 32];

        for i in (0..n).rev() {
            let mut max_index = i as i32;
            for bit in 0..32 {
                let bit_set = nums[i] & (1 << bit) != 0;
                if bit_set {
                    last_seen[bit] = i as i32;
                } else {

                    if last_seen[bit] != -1 {
                        max_index = max_index.max(last_seen[bit]);
                    }

                }
            }
            result[i] = max_index - i as i32 + 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 3, 2, 2, 1],
            Solution::smallest_subarrays(vec![1, 0, 2, 1, 3])
        );
        assert_eq!(vec![2, 1], Solution::smallest_subarrays(vec![1, 2]));
    }
}
