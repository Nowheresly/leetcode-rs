
pub struct Solution {}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    continue;
                }
                ret += 1;
                for k in 2.. (matrix.len() - i+1) {
                    if check_square(&matrix, i, j, k) {
                        ret += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        ret
    }
}

fn check_square(matrix: &Vec<Vec<i32>>, row: usize, col: usize, size:usize) -> bool {
    if col + size -1 >= matrix[0].len() {
        return false;
    }
    for i in row..(row + size) {
        for j in col..(col + size) {
            if matrix[i as usize][j as usize] == 0 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(15, Solution::count_squares(vec![
            vec![0,1,1,1],
            vec![1,1,1,1],
            vec![0,1,1,1]
        ]));

    }
}
