pub struct Solution {}

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid[0].len();
        let mut cur = vec![vec![0;2];n];
        let mut prev = vec![vec![0;2];n];
        let mut res = 0;
        cur[0][0] = if grid[0][0] == 'X' {1} else {0};
        cur[0][1] = if grid[0][0] == 'Y' {1} else {0};

        for i in 1..n {
            cur[i][0] += cur[i-1][0] + if grid[0][i] == 'X' {1} else {0};
            cur[i][1] += cur[i-1][1] + if grid[0][i] == 'Y' {1} else {0};

            if cur[i][0] > 0 && cur[i][0] == cur[i][1] {
                res += 1;
            }
        }

        for row in 1..grid.len() {
            prev = cur.clone();
            cur = vec![vec![0;2];n];
            for i in 0..n {
                if i > 0 {
                    cur[i][0] += cur[i-1][0] - prev[i-1][0];
                    cur[i][1] += cur[i-1][1] - prev[i-1][1];
                }
                cur[i][0] += prev[i][0] + if grid[row][i] == 'X' {1} else {0};
                cur[i][1] += prev[i][1] + if grid[row][i] == 'Y' {1} else {0};

                if cur[i][0] > 0 && cur[i][0] == cur[i][1] {
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
        assert_eq!(3, Solution::number_of_submatrices(vec![vec!['X','Y','.'],vec!['Y','.','.']]));

    }
}