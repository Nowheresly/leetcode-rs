pub struct Solution {}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        let mut c = vec![true; 4];
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] != target[i][j] {
                    c[0] = false;
                }
                if mat[n - 1 - j][i] != target[i][j] {
                    c[1] = false;
                }
                if mat[n - 1 - i][n - 1 - j] != target[i][j] {
                    c[2] = false;
                }
                if mat[j][n - 1 - i] != target[i][j] {
                    c[3] = false;
                }
            }
        }
        c[0] || c[1] || c[2] || c[3]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::find_rotation(
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
                vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
            )
        );
    }
}
