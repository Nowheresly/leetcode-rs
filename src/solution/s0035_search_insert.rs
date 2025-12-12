pub struct Solution {}


impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut start:i32 = 0;
        let mut end:i32 = n as i32 - 1;
        loop {
            if end <= start {
                if target <= nums[start as usize] {
                    return start as i32;

                }
                return start as i32 + 1;
            }
            let ret:i32 = start + (end - start) / 2;
            let cur_val = nums[ret as usize];
            if cur_val == target {
                return ret as i32;
            }
            if cur_val < target {
                start = ret + 1;
            } else {
                end = ret - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::search_insert(vec![1,3,5,6], 5));
    }
}
