
pub struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let n = arr.len();
        let mut ans = 0;
        let mut sum = 0;
        let mut l = 0;
        for r in 0..n {
            sum += arr[r];
            if r - l + 1 < k as usize {
                continue;
            }
            if r - l + 1 > k as usize {
                sum -= arr[l];
                l += 1;
            }
            if sum / k >= threshold {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_of_subarrays(vec![2,2,2,2,5,5,5,8], 3, 4));
    }
}
