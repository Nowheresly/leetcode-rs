
pub struct Solution {}

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let m = mat.len();
        let n = mat[0].len();

        let k = k as usize % n;

        if k == 0 {
            return true;
        }

        for i in 0..m {
            for j in 0..n {
                if i % 2 == 0 {
                    if mat[i][j] != mat[i][(j + k) % n] {
                        return false;
                    }
                } else {
                    if mat[i][j] != mat[i][(j - k + n) % n] {
                        return false;
                    }
                }
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
        assert_eq!(false, Solution::are_similar(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 4));
    }
}
