use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = vec![-1; n];
        let mut right = vec![n as i32; n];
        let mut stack = vec![];
        // Calculate left limits
        for i in 0..n {
            while !stack.is_empty() && arr[*stack.last().unwrap()] >= arr[i] {
                stack.pop();
            }
            if stack.is_empty() == false {  // If stack is not empty, get the last element
                left[i] = *stack.last().unwrap() as i32;
            }

            stack.push(i);
        }
        stack.clear();
        // Calculate right limits
        for i in (0..n).rev() {
            while !stack.is_empty() && arr[*stack.last().unwrap()] > arr[i] {
                stack.pop();
            }
            if stack.is_empty() == false {  // If stack is not empty, get the last element
                right[i] = *stack.last().unwrap() as i32;
            }
            stack.push(i);
        }
        let mut result:i64 = 0;
        for i in 0..n {
            let mut left = i as i32 - left[i]; // Calculate the number of elements to the left
            let mut right = right[i] - i as i32; // Calculate the number of elements to the right
            let mut add: i64 = ((left as i64 * right as i64 ) % 1_000_000_007 * arr[i] as i64);
            result = (result + add) % 1_000_000_007;
            result %= 1_000_000_007;
        }
        (result % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(17, Solution::sum_subarray_mins(vec![3,1,2,4]));
        assert_eq!(444, Solution::sum_subarray_mins(vec![11,81,94,43,3]));

    }
}
