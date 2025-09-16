
pub struct Solution {}

impl Solution {
    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        let mut cnt = 0;
        for n in nums.iter() {
            if *n > target {
                break;
            }
            if *n == target {
                cnt += 1;
            }
        }
        cnt > nums.len() / 2
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_majority_element(vec![2,4,5,5,5,5,5,6,6], 5));
        assert_eq!(false, Solution::is_majority_element(vec![10,100,101,101], 101));
    }
}
