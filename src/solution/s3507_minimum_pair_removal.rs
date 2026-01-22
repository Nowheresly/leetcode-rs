pub struct Solution {}

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut res = 0;

        while non_decreasing(&nums) == false {
            let idx = find_smallest_pair(&nums);
            let sum = nums[idx] + nums[idx+1];
            res += 1;
            nums.remove(idx+1);
            nums.remove(idx);
            nums.insert(idx, sum);
        }
        res
    }
}

fn find_smallest_pair(nums: &Vec<i32>) -> usize {
    let mut min = i32::MAX;
    let mut res = 0;
    for i in 0..nums.len()-1 {
        let sum = nums[i] + nums[i+1];
        if sum < min {
            min = sum;
            res = i;
        }
    }
    res
}

fn non_decreasing(nums: &Vec<i32>) -> bool {
    for i in 1..nums.len() {
        if nums[i] < nums[i-1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2, Solution::minimum_pair_removal(vec![5,2,3,1]));
    }
}
