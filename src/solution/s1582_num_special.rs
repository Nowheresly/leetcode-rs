
pub struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    let mut row_sum = 0;
                    let mut col_sum = 0;
                    for k in 0..mat.len() {
                        row_sum += mat[k][j];
                    }
                    for k in 0..mat[i].len() {
                        col_sum += mat[i][k];
                    }
                    if row_sum == 1 && col_sum == 1 {
                        ret += 1;
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::num_special(vec![vec![1,0,0],vec![0,0,1],vec![1,0,0]]));
        assert_eq!(3, Solution::num_special(vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]]));
    }
}
