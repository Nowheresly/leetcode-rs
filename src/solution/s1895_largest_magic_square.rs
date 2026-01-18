pub struct Solution {}

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let max_square_side= n.min(m);
        let mut res = 1;
        for k in (2..=max_square_side).rev() {
            for row in 0..=(m-k) {
                for col in 0..=(n-k) {
                    if is_magic(&grid, row, col, k) {
                        return k as i32;
                    }
                }
            }
        }
        res
    }
}

fn is_magic(grid: &Vec<Vec<i32>>, row:usize, col:usize, k:usize) -> bool {
    // compute diag sum
    let mut sum_diag1 = 0;
    let mut sum_diag2 = 0;
    for i in 0..k {
        sum_diag1 += grid[row+i][col+i];
        sum_diag2 += grid[row+i][col+k-1-i];
    }
    if sum_diag1 != sum_diag2 {
        return false;
    }
    // check rows and cols
    for i in 0..k {
        let mut sum_row = 0;
        let mut sum_col = 0;
        for j in 0..k {
            sum_row += grid[row+i][col+j];
            sum_col += grid[row+j][col+i];
        }
        if sum_row != sum_diag1 || sum_col != sum_diag1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::largest_magic_square(vec![vec![7,1,4,5,6],vec![2,5,1,6,4],vec![1,5,4,3,2],vec![1,2,7,3,4]]));
    }
}
