
pub struct Solution {}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let n = matrix.len();
        let mut res:i64 = 0;
        let mut cnt_neg = 0;
        let mut mat_min = 100_001;
        for r in 0..n {
            for c in 0..n {
                mat_min = mat_min.min(matrix[r][c].abs());
                res += matrix[r][c].abs() as i64;
                if matrix[r][c] < 0 {
                    cnt_neg += 1;
                }
            }
        }
        if cnt_neg % 2 == 0 {
            return res;
        }
        res - 2 * mat_min as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(4, Solution::max_matrix_sum(vec![vec![1,-1],vec![-1,1]]));
        assert_eq!(16, Solution::max_matrix_sum(vec![vec![1,2,3],vec![-1,-2,-3],vec![1,2,3]]));

    }
}
