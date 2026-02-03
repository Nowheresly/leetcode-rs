pub struct Solution {}

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut state = 0;
        let n = nums.len();
        let mut count = 0;
        for i in 1..n {
            let val = nums[i];
            let prev = nums[i-1];
            if prev == val {
                return false;
            }
            if state == 0 {
                if prev < val {
                    count+=1;
                    continue;
                }
                if count == 0 {
                    return false;
                }
                count = 1;
                state = 1;
                continue;
            }
            if state == 1 {
                if prev > val {
                    continue;
                }
                state = 2 ;
            }
            if state == 2 {
                if prev < val {
                    count += 1;
                    continue;
                }
                return false;
            }
        }
        if state != 2 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]));
    }
}
