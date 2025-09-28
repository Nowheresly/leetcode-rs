
pub struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        for i in (2..n).rev() {
            let a = nums[i - 2];
            let b = nums[i - 1];
            let c = nums[i];
            if a + b > c {
                return a + b + c;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::largest_perimeter(vec![2,1,2]));
        assert_eq!(0, Solution::largest_perimeter(vec![1,2,1,10]));

    }
}
