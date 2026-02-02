use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_heap = BinaryHeap::new();
        for i in 1..n {
            if max_heap.len() < 2 {
                max_heap.push(nums[i]);
                continue;
            }
            if max_heap.peek().unwrap() < &nums[i] {
                continue;
            }
            max_heap.push(nums[i]);
            max_heap.pop();
        }
        nums[0] + max_heap.pop().unwrap() + max_heap.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::minimum_cost(vec![1,2,3,12]));

    }
}
