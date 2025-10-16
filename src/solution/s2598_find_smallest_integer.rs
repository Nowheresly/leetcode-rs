pub struct Solution {}

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut map = vec![0; value as usize];
        for num in nums {
            let key = ((num % value) + value) % value;
            map[key as usize] += 1;
        }
        let mut ret = 0;
        loop {
            for i in 0..value {
                if map[i as usize] > 0 {
                    map[i as usize] -= 1;
                    ret += 1;
                } else {
                    return ret;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::find_smallest_integer(vec![1,-10,7,13,6,8], 5));
        assert_eq!(2, Solution::find_smallest_integer(vec![1,-10,7,13,6,8], 7));
    }
}