pub struct Solution {}

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ret_mat = mat.clone();
        for row in 0..m {
            let mut vec = vec![];
            for i in 0..n {
                if i + row >= m {
                    break;
                }
                vec.push(mat[row + i][i]);
            }
            vec.sort();
            for i in 0..n {
                if i + row >= m {
                    break;
                }
                ret_mat[row + i][i] = vec[i];
            }
        }
        for col in 1..n {
            let mut vec = vec![];
            for i in 0..m {
                if i + col >= n {
                    break;
                }
                vec.push(mat[i][col + i]);
            }
            vec.sort();
            for i in 0..m {
                if i + col >= n {
                    break;
                }
                ret_mat[i][col + i] = vec[i];
            }
        }
        ret_mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]],
            Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]])
        );
        assert_eq!(
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68]
            ],
            Solution::diagonal_sort(vec![
                vec![11, 25, 66, 1, 69, 7],
                vec![23, 55, 17, 45, 15, 52],
                vec![75, 31, 36, 44, 58, 8],
                vec![22, 27, 33, 25, 68, 4],
                vec![84, 28, 14, 11, 5, 50]
            ])
        );
    }
}
