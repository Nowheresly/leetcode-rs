pub struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut pstart:i32 = 0;
        let mut pend:i32 = nums.len() as i32 - 1;
        let mut k = 0;
        while pstart <= pend {
            if nums[pstart as usize] != val {
                pstart += 1;
                k += 1;
                continue;
            }
            nums[pstart as usize] = nums[pend as usize];
            pend -= 1;
        }
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::remove_element(&mut vec![3,2,2,3], 3));


    }
}
