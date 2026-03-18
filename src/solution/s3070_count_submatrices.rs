
pub struct Solution {}

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut res = 1;
        if grid[0][0] > k {
            return 0;
        }
        let mut grid = grid;
        for j in 1..grid[0].len() {
            grid[0][j] += grid[0][j-1];
            if grid[0][j] <= k {
                res += 1;
            }
        }

        for i in 1..grid.len() {
            for j in 0..grid[0].len() {
                if j > 0 {
                    grid[i][j] += grid[i][j-1];
                    grid[i][j] -= grid[i - 1][j-1];
                }
                grid[i][j] += grid[i-1][j];
                if grid[i][j] <= k {
                    res += 1;
                }
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
        assert_eq!(6, Solution::count_submatrices(vec![vec![7,2,9],vec![1,5,0],vec![2,6,6]], 20));

    }
}
