use std::collections::{HashSet};

pub struct Solution {}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = grid.len();
        let m = grid[0].len();
        if n < 3 || m < 3 {
            return 0;
        }
        for row in 0..=n-3 {
            for col in 0..=m-3 {
                if is_valid(row, col, &grid) {
                    res += 1;
                }
            }
        }
        res
    }
}

fn is_valid(row: usize, col: usize, grid: &Vec<Vec<i32>>) -> bool {
    let sum = grid[row][col] + grid[row + 1][col] + grid[row + 2][col];
    // check row
    for i in row..row+3 {
        let mut tmp = 0;
        for j in col..col+3 {
            tmp += grid[i][j];
        }
        if tmp != sum {
            return false;
        }
    }

    // check col + distinct
    let mut set = HashSet::new();
    for j in col..col+3 {
        let mut tmp = 0;
        for i in row..row+3 {
            if grid[i][j] == 0 || grid[i][j] > 9 {
                return false;
            }
            tmp += grid[i][j];
            set.insert(grid[i][j]);
        }
        if tmp != sum {
            return false;
        }
    }
    if set.len() < 9 {
        return false;
    }
    // chck diag
    let diag1 = grid[row][col] + grid[row+1][col+1] + grid[row+2][col+2];
    if diag1 != sum {
        return false;
    }
    let diag2 = grid[row+2][col] + grid[row+1][col+1] + grid[row][col+2];
    if diag2 != sum {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        //assert_eq!(1, Solution::num_magic_squares_inside(vec![vec![4,3,8,4],vec![9,5,1,9],vec![2,7,6,2]]));
        assert_eq!(0, Solution::num_magic_squares_inside(vec![vec![8]]));

    }
}
