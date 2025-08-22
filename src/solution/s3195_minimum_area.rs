pub struct Solution {}

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut min_row = i32::MAX;
        let mut min_col = i32::MAX;
        let mut max_row = 0;
        let mut max_col = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    min_row = min_row.min(i as i32);
                    min_col = min_col.min(j as i32);
                    max_row = max_row.max(i as i32);
                    max_col = max_col.max(j as i32);
                }
            }
        }
        if min_col == i32::MAX {
            return 0;
        }
        (max_col - min_col + 1) * (max_row - min_row + 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::minimum_area(vec![
            vec![0, 1, 0],
            vec![1, 0, 1]
        ]));
    }
}