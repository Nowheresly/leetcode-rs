pub struct Solution {}

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        for i in 0..n {
            let n = n as i32;
            let a = ((i as i32 + nums[i] % n) +  n) % n;
            res[i] = nums[a as usize];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1,1,1,3], Solution::construct_transformed_array(vec![3,-2,1,1]));
        assert_eq!(vec![-1,-1,4], Solution::construct_transformed_array(vec![-1,4,-1]));

    }
}
