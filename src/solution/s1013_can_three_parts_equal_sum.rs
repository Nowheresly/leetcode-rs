
pub struct Solution {}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut sum = 0i64;
        for i in arr.iter() {
            sum += *i as i64;
        }
        if sum % 3 != 0 {
            return false;
        }
        let target = sum / 3;
        let mut sumi = 0i64;
        for i in 0..arr.len() {
            sumi += arr[i] as i64;
            if sumi == target {
                let mut sumj = 0i64;
                for j in (i+1)..arr.len() {
                    sumj += arr[j] as i64;
                    if sumj == target {
                        return j < arr.len() - 1;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_three_parts_equal_sum(vec![0,2,1,-6,6,-7,9,1,2,0,1]));


    }
}
