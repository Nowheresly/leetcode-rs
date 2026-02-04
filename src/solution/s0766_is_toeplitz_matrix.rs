

pub struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        if m == 1 || n == 1 {
            return true;
        }
        for i in 0..m {
            let val = matrix[i][0];
            let mut a = 1;
            while i+a < m && a < n {
                if val != matrix[i+a][a] {
                    return false;
                }
                a += 1;
            }
        }

        for i in 0..n {
            let val = matrix[0][i];
            let mut a = 1;
            while i+a < n && a < m {
                if val != matrix[a][i+a] {
                    return false;
                }
                a += 1;
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
        assert_eq!(
            true,
            Solution::is_toeplitz_matrix(vec![vec![1,2,3,4],vec![5,1,2,3],vec![9,5,1,2]])
        );
    }
}
