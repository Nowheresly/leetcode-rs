pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut count_even = 0;
        let mut count_odd = 0;
        for num in nums.iter() {
            if num % 2 == 0 {
                count_even += 1;
            } else {
                count_odd += 1;
            }
        }
        ret = count_odd.max(count_even);
        let mut even = true;
        let mut count_even_odd = 0;
        for num in nums.iter() {
            if even == (num % 2 == 0) {
                count_even_odd += 1;
                even = !even;
            }
        }
        ret = ret.max(count_even_odd);
        even = false;
        let mut count_odd_even = 0;
        for num in nums.iter() {
            if even == (num % 2 == 0) {
                count_odd_even += 1;
                even = !even;
            }
        }
        ret = ret.max(count_odd_even);
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::maximum_length(vec![1,2,3,4]));
        assert_eq!(6, Solution::maximum_length(vec![1,2,1,1,2,1,2]));

    }
}