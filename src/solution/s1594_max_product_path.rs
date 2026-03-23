
pub struct Solution {}

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp_min = vec![vec![0i64; n]; m];
        let mut dp_max = vec![vec![0i64; n]; m];
        dp_min[0][0] = grid[0][0] as i64;
        dp_max[0][0] = grid[0][0] as i64;

        for row in 0..m {
            for col in 0..n {
                if row == 0 && col == 0 {
                    continue;
                }
                if row == 0 {
                    let val1 = grid[row][col] as i64 * dp_min[row][col-1];
                    let val2 = grid[row][col] as i64 * dp_max[row][col-1];
                    dp_min[row][col] = val1.min(val2);
                    dp_max[row][col] = val1.max(val2);
                    continue;
                }
                if col == 0 {
                    let val1 = grid[row][col] as i64 * dp_min[row-1][col];
                    let val2 = grid[row][col] as i64 * dp_max[row-1][col];
                    dp_min[row][col] = val1.min(val2);
                    dp_max[row][col] = val1.max(val2);
                    continue;
                }
                let val1 = grid[row][col] as i64 * dp_min[row-1][col];
                let val2 = grid[row][col] as i64 * dp_max[row-1][col];
                let val3 = grid[row][col] as i64 * dp_min[row][col-1];
                let val4 = grid[row][col] as i64 * dp_max[row][col-1];
                dp_min[row][col] = val1.min(val2).min(val3).min(val4);
                dp_max[row][col] = val1.max(val2).max(val3).max(val4);
            }
        }
        if dp_max[m-1][n-1] < 0 {
            return -1;
        }
        (dp_max[m-1][n-1] % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::max_product_path(vec![vec![1,-2,1],vec![1,-2,1],vec![3,-4,1]]));
    }
}
