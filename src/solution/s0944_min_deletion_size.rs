pub struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut grid:Vec<Vec<char>> = vec![vec!['a'; strs[0].len()]; strs.len()];
        for i in 0..strs.len() {
            for j in 0..strs[i].len() {
                grid[i][j] = strs[i].chars().nth(j).unwrap_or('a');
            }
        }
        let mut ans = 0;
        for c in 0..grid[0].len() {
            for r in 1..grid.len() {
                if grid[r - 1][c] > grid[r][c] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::min_deletion_size(vec![
                String::from("cba"),
                String::from("daf"),
                String::from("ghi")
            ])
        );
    }
}
