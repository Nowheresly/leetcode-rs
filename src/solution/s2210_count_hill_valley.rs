
pub struct Solution {}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 1..nums.len() {
            let val = nums[i];
            if val == nums[i - 1] {
                continue;
            }
            let mut prev = val;
            for j in (0..i).rev() {
                prev = nums[j];
                if val != prev {
                    break;
                }
            }
            if prev == val {
                continue;
            }
            let mut next = val;
            for j in i + 1..nums.len() {
                next = nums[j];
                if val != next {
                    break;
                }
            }
            if next == val {
                continue;
            }
            if (prev < val && next < val) || (prev > val && next > val) {
                ret += 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_hill_valley(vec![2,4,1,1,6,5]));
        assert_eq!(0, Solution::count_hill_valley(vec![6,6,5,5,4,1]));
    }
}
