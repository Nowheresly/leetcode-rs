use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        for i in 0..n {
            let mut set1 = HashSet::new();
            let mut set2 = HashSet::new();

            for j in 0..n {
                if set1.contains(&matrix[i][j]) || set2.contains(&matrix[j][i]) {
                    return false;
                }
                set1.insert(matrix[i][j]);
                set2.insert(matrix[j][i]);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::check_valid(vec![vec![1,2,3],vec![3,1,2],vec![2,3,1]]));
        assert_eq!(false, Solution::check_valid(vec![vec![1,1,1],vec![1,2,3],vec![1,2,3]]));
    }
}
