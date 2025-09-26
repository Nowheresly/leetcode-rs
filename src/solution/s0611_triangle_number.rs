
pub struct Solution {}

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        for i in 0..n-2 {
            for j in i+1..n-1 {
                let sum = nums[i] + nums[j];
                let mut left = j + 1;
                let mut right = n - 1;
                while left <= right {
                    let mid = left + (right - left) / 2;
                    if nums[mid] < sum {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
                res += (left - j - 1) as i32;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        //assert_eq!(3, Solution::triangle_number(vec![2,2,3,4]));
        assert_eq!(0, Solution::triangle_number(vec![1]));

    }
}
