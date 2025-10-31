

pub struct Solution {}

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set = vec![0; nums.len()];
        let mut dups:Vec<i32> = vec![];
        for i in nums.iter() {
            if set[*i as usize] > 0 {
                dups.push(*i);
                if dups.len() == 2 {
                    break;
                }
            }
            set[*i as usize] += 1;
        }
        dups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 0], Solution::get_sneaky_numbers(vec![0, 1, 1, 0]));
        assert_eq!(
            vec![3, 2],
            Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2])
        );
        assert_eq!(
            vec![4, 5],
            Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2])
        );
    }
}
