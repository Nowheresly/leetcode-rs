pub struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut freq = vec![0; n + 1];

        for &num in &arr {
            if num as usize <= n {
                freq[num as usize] += 1;
            }
        }
        let mut ret: i32 = -1;
        for (i, val) in freq.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if *val == (i as i32) {
                ret = *val;
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_lucky(vec![2, 2, 3, 4]));
        assert_eq!(3, Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]));
        assert_eq!(-1, Solution::find_lucky(vec![2, 2, 2, 3, 3]));
    }
}
