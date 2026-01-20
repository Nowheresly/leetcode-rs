pub struct Solution {}

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        for i in 0..n {
            let x = nums[i];
            let mut res = -1;
            let mut d = 1;
            while ( x & d) != 0 {
                res = x - d;
                d <<= 1;
            }
            result[i] = res;
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![-1,1,4,3], Solution::min_bitwise_array(vec![2,3,5,7]));

    }
}