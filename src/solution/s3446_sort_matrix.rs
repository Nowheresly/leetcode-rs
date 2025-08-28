pub struct Solution {}

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut grid_ret = grid.clone();
        for row in 0..n {
            let mut vec = vec![];
            for i in 0..(n - row) {
                vec.push(grid[row + i][i]);
            }
            vec.sort_by(|a, b| b.cmp(a));
            for i in 0..(n - row) {
                grid_ret[row + i][i] = vec[i];
            }
        }

        for col in 1..n {
            let mut vec = vec![];
            for i in 0..(n - col) {
                vec.push(grid[i][col + i]);
            }
            vec.sort();
            for i in 0..(n - col) {
                grid_ret[i][col + i] = vec[i];
            }
        }

        grid_ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]], Solution::sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]]));
        assert_eq!(vec![vec![2, 1], vec![1, 0]], Solution::sort_matrix(vec![vec![0, 1], vec![1, 2]]));
    }
}
