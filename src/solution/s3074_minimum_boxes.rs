
pub struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut capacity = capacity;
        capacity.sort();
        let n = apple.len();
        let mut res = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += apple[i];
        }
        for i in (0..capacity.len()).rev() {
            sum -= capacity[i];
            res += 1;
            if sum <= 0 {
                break;
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
        assert_eq!(2, Solution::minimum_boxes(vec![1,3,2], vec![4,3,1,5,2]));
    }
}
