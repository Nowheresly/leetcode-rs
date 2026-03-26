pub struct Solution {}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut total:i64 = 0;
        let mut max_val:i32 = 0;
        for i in 0..m {
            for j in 0..n {
                total += grid[i][j] as i64;
                max_val = max_val.max(grid[i][j]);
            }
        }

        let mut freqa = vec![0; max_val as usize +1];
        let mut freqb = vec![0; max_val as usize +1];

        // --- Check all Horizontal Cuts ---
        // Initially, section B holds the entire grid.
        for i in 0..m {
            for j in 0..n {
                freqb[grid[i][j] as usize] += 1;
            }
        }

        let mut suma:i64 = 0;
        for r in 1..m {
            for j in 0..n {
                let val = grid[r - 1][j];
                freqb[val as usize] -= 1;
                freqa[val as usize] += 1;
                suma += val as i64;
            }

            let sumb = total - suma;
            if suma == sumb {
                return true;
            }

            if suma > sumb {
                let diff = suma - sumb;
                if diff <= max_val as i64 {
                    let x = diff as i32;
                    if n == 1 {
                        if grid[0][0] == x || grid[r-1][0] == x {return true;}
                    } else if r == 1 {
                        if grid[0][0] == x || grid[0][n-1] == x {return true;}
                    } else {
                        if freqa[x as usize]>0 { return true;}
                    }
                }
            } else {
                let diff = sumb - suma;
                if diff <= max_val as i64 {
                    let x = diff as i32;
                    if n == 1 {
                        if grid[r][0] == x || grid[m-1][0] == x { return true;}
                    } else if m - r == 1 {
                        if grid[m - 1][0] == x || grid[m-1][n-1] == x { return true;}
                    } else {
                        if freqb[x as usize] > 0 { return true;}
                    }
                }
            }
        }

        let mut freqa = vec![0; max_val as usize +1];
        let mut freqb = vec![0; max_val as usize +1];

        for i in 0..m {
            for j in 0..n {
                freqb[grid[i][j] as usize] += 1;
            }
        }
        let mut suma = 0;
        for c in 1..n {
            for i in 0..m {
                let val = grid[i][c-1];
                freqb[val as usize] -= 1;
                freqa[val as usize] += 1;
                suma += val as i64;
            }

            let sumb = total - suma;
            if suma == sumb {
                return true;
            }

            if suma > sumb {
                let diff = suma - sumb;
                if diff <= max_val as i64 {
                    let x = diff as i32;
                    if m == 1 {
                        if grid[0][0] == x || grid[0][c - 1] == x { return true; }
                    } else if c == 1 {
                        if grid[0][0] == x || grid[m - 1][0] == x { return true; }
                    } else {
                        if freqa[x as usize] > 0 { return true; }
                    }
                }
            } else {
                let diff = sumb - suma;
                if diff <= max_val as i64 {
                    let x = diff as i32;
                    if m == 1 {
                        if grid[0][c] == x || grid[0][n - 1] == x { return true; }
                    } else if n-c == 1 {
                        if grid[0][n-1] == x || grid[m-1][n-1] == x { return true;}
                    } else {
                        if freqb[x as usize] > 0 { return true; }
                    }
                }
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
