use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let mut treeset: BTreeSet<i32> = BTreeSet::new();
        for row in 0..m {
            for col in 0..n {
                treeset.insert(grid[row as usize][col as usize]);
                if treeset.len() > 3 {
                    treeset.pop_first();
                }
                let mut size = 1;
                while row + size < m && row - size >= 0 && col + 2 * size < n {
                    let mut sum = grid[row as usize][col as usize]
                        + grid[row as usize][col as usize + 2 * size as usize];
                    sum += grid[row as usize + size as usize][col as usize + size as usize];
                    sum += grid[row as usize - size as usize][col as usize + size as usize];

                    for i in 1..size {
                        sum += grid[row as usize + i as usize][col as usize + i as usize];
                        sum += grid[row as usize - i as usize][col as usize + i as usize];
                        sum += grid[row as usize + size as usize - i as usize]
                            [col as usize + size as usize + i as usize];
                        sum += grid[row as usize - size as usize + i as usize]
                            [col as usize + size as usize + i as usize];
                    }

                    treeset.insert(sum);
                    if treeset.len() > 3 {
                        treeset.pop_first();
                    }
                    size += 1;
                }
            }
        }
        let mut res = vec![0; treeset.len()];
        let mut i = 0;
        while treeset.is_empty() == false {
            res[i] = treeset.pop_last().unwrap();
            i += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![228, 216, 211],
            Solution::get_biggest_three(vec![
                vec![3, 4, 5, 1, 3],
                vec![3, 3, 4, 2, 3],
                vec![20, 30, 200, 40, 10],
                vec![1, 5, 5, 4, 1],
                vec![4, 3, 2, 2, 5]
            ])
        );
    }
}
