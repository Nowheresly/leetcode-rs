pub struct Solution {}

impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = vec![0;101];
        for i in nums {
            freq[i as usize]+= 1;
        }
        let mut res:i32 = 0;
        for i in 1..101 {
            if freq[i] > 0 && freq[i] % k == 0 {
                res += i as i32 * freq[i];
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
        assert_eq!(16, Solution::sum_divisible_by_k(vec![1,2,2,3,3,3,3,4], 2));

    }
}
