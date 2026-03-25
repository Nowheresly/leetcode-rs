pub struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut sum_rows = vec![0;m];
        let mut total = 0i64;
        for i in 0..m {
            for j in 0..n {
                sum_rows[i] += grid[i][j] as i64;
                total += grid[i][j] as i64;
            }
        }
        if total % 2 == 1 {
            return false;
        }
        let part = total / 2;
        let mut current = 0;
        for i in 0..m {
            current += sum_rows[i];
            if current == part {
                return true;
            }
            if current > part {
                break;
            }
        }
        current = 0;
        for j in 0..n {
            for i in 0..m {
                current += grid[i][j] as i64;
            }
            if current == part {
                return true;
            }
            if current > part {
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_partition_grid(vec![vec![1,4],vec![2,3]]));

    }
}
