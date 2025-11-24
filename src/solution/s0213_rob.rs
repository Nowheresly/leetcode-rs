pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        let max1 = rob_simple(&nums, 0, n - 2);
        let max2 = rob_simple(&nums, 1, n - 1);

        max1.max(max2)
    }
}

fn rob_simple(nums: &[i32], start: usize, end: usize) -> i32 {
    let mut t1 = 0;
    let mut t2 = 0;
    for i in start..=end {
        let tmp = t1;
        let current = nums[i];
        t1 = t1.max(t2 + current);
        t2 = tmp;
    }
    t1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::rob(vec![2,3,2]));
        assert_eq!(4, Solution::rob(vec![1,2,3,1]));
        assert_eq!(3, Solution::rob(vec![1,2,3]));

    }
}
