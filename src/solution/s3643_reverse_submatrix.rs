pub struct Solution {}

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = grid.clone();
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        for i in 0..k / 2 {
            let row_from = x + i;
            let row_to = x + k - 1 - i;

            for col in y..y + k {
                let tmp = res[row_from][col];
                res[row_from][col] = res[row_to][col];
                res[row_to][col] = tmp
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                vec![1, 2, 3, 4],
                vec![13, 14, 15, 8],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 16]
            ],
            Solution::reverse_submatrix(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16]
                ],
                1,
                0,
                3
            )
        );
    }
}
