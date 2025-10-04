
pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max = 0;
        while left < right {
            let h = height[left].min(height[right]);
            max = max.max(h * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
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
        assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
        assert_eq!(1, Solution::max_area(vec![1,1]));

    }
}
