pub struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let max_val = *nums.iter().max().unwrap() + k + 2;
        let mut count = vec![0; max_val as usize];
        for v in nums.iter() {
            count[*v as usize] += 1;
        }
        for i in 1..max_val {
            count[i as usize] += count[(i - 1) as usize];
        }
        let mut res = 0;
        for i in 0..max_val {
            let left = (i - k).max(0);
            let right = (i + k).min(max_val - 1);
            let total = count[right as usize]
                - if left > 0 {
                    count[(left - 1) as usize]
                } else {
                    0
                };
            let freq = count[i as usize] - if i > 0 { count[(i - 1) as usize] } else { 0 };
            res = res.max(freq + num_operations.min(total - freq));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::max_frequency(vec![1, 4, 5], 1, 2));
        assert_eq!(2, Solution::max_frequency(vec![5,11,20,20], 5, 1));
    }
}
