
pub struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let m = grid.len();
        let n = grid[0].len();
        for r in 0..m {
            let first = grid[r][0];
            if first < 0 {
                res += (m - r) * n;
                break;
            }
            for c in 0..n {
                let val = grid[r][c];
                if val < 0 {
                    res += n - c;
                    break;
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::count_negatives(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]));
    }
}
