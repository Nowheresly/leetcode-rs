pub struct Solution {}

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = vec![vec![0;m]; n];
        let modu = 12345;

        let mut running_prefix:i64 = 1;
        for i in 0..n {
            for j in 0..m {
                ans[i][j] = running_prefix as i32;
                running_prefix = (running_prefix * (grid[i][j] as i64 % modu)) % modu;
            }
        }
        let mut running_suffix:i64 = 1;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                ans[i][j] = ((ans[i][j] as i64 * running_suffix) % modu) as i32;
                running_suffix = (running_suffix * (grid[i][j] as i64 % modu)) % modu;
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
        assert_eq!(vec![vec![24,12],vec![8,6]], Solution::construct_product_matrix(vec![vec![1,2],vec![3,4]]));

    }
}
