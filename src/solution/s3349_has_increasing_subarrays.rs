pub struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let k = k as usize;
        let mut start = 0;
        for a in 1..nums.len()-k {
            let b = a + k;
            let preva = nums[a-1];
            let prevb = nums[b-1];
            let vala = nums[a];
            let valb = nums[b];
            if preva < vala && prevb < valb {
                if a -start + 1 == k {
                    return true;
                }
                continue;
            }
            start = a;
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::has_increasing_subarrays(vec![2,5,7,8,9,2,3,4,3,1], 3));
        assert_eq!(false, Solution::has_increasing_subarrays(vec![1,2,3,4,4,4,4,5,6,7], 5));

    }
}