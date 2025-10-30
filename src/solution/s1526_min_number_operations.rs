pub struct Solution {}

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut res = target[0];
        for i in 1..target.len() {
            let val = target[i];
            let prev = target[i - 1];
            if val == prev {
                continue;
            }
            if val > prev {
                res += val - prev;
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
        assert_eq!(3, Solution::min_number_operations(vec![1,2,3,2,1]));
        assert_eq!(4, Solution::min_number_operations(vec![3,1,1,2]));
        assert_eq!(7, Solution::min_number_operations(vec![3,1,5,4,2]));

    }
}
