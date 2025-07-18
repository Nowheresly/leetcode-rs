use std::collections::BinaryHeap;
use std::cmp::Reverse;
pub struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let k = n / 3;
        let mut sum_left:i64 = 0;
        let mut left_sums:Vec<i64> = vec![0; n];
        let mut left_heap = BinaryHeap::new();
        for i in 0..k {
            sum_left += nums[i] as i64;
            left_heap.push(nums[i] as i64);
        }
        left_sums[k - 1] = sum_left;
        for i in k..(n - k) {
            let x = nums[i] as i64;
            if x < *left_heap.peek().unwrap() {
                sum_left += x - left_heap.pop().unwrap();
                left_heap.push(x);
            }
            left_sums[i] = sum_left;
        }

        let mut sum_right:i64 = 0;
        let mut right_sums = vec![0; n];
        let mut right_heap = BinaryHeap::new();
        for i in ((n-k)..n).rev() {
            sum_right += nums[i] as i64;
            right_heap.push(Reverse(nums[i] as i64));
        }
        right_sums[n - k] = sum_right;
        for i in ((k-1)..(n-k)).rev() {
            let x = nums[i] as i64;
            if x > right_heap.peek().unwrap().0 {
                sum_right += x - right_heap.pop().unwrap().0;
                right_heap.push(Reverse(x));
            }
            right_sums[i] = sum_right;
        }
        let mut min_diff:i64 = i64::MAX;
        for i in k-1..(n-k) {
            let diff = left_sums[i] - right_sums[i+1];
            min_diff = min_diff.min(diff);
        }
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(-1, Solution::minimum_difference(vec![3,1,2]));
        assert_eq!(1, Solution::minimum_difference(vec![7,9,5,8,1,3]));
    }
}
