
pub struct Solution {}

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut low = 0;
        let mut high = cells.len() - 1;
        let mut res: i32 = 0;
        while low <= high {
            let mid = (low + high) / 2;
            if can_cross(row, col, &cells, mid) {
                res = mid as i32;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        res + 1
    }
}

fn can_cross(row: i32, col: i32, cells: &[Vec<i32>], max: usize) -> bool {
    let mut grid = vec![vec![0; col as usize]; row as usize];
    for i in 0..=max {
        let c = &cells[i];
        grid[c[0] as usize - 1][c[1] as usize - 1] = 1;
    }
    for i in 0..col {
        let mut visited = vec![vec![false; col as usize]; row as usize];
        let ret = dfs(&mut grid, 0, i, &mut visited);
        if ret {
            return true;
        }
    }
    false
}
const DX: [i32; 4] = [0,-1,0,1];
const DY: [i32; 4] = [-1,0,1, 0];

fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, visited: &mut Vec<Vec<bool>>) -> bool {
    if row < 0 || col < 0  || row == grid.len() as i32 || col == grid[0].len() as i32 || visited[row as usize][col as usize] {
        return false;
    }
    if grid[row as usize][col as usize] > 0 {
        return false;
    }
    if row == grid.len() as i32 - 1 {
        return true;
    }
    visited[row as usize][col as usize] = true;
    for i in 0..4 {
        let nrow = row + DX[i];
        let ncol = col + DY[i];
        if dfs(grid, nrow, ncol, visited) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(3, Solution::latest_day_to_cross(3,3, vec![vec![1,2],vec![2,1],vec![3,3],vec![2,2],vec![1,1],vec![1,3],vec![2,3],vec![3,2],vec![3,1]]));
    }
}
