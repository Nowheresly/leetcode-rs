pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len() as i32;
        let n = heights[0].len() as i32;
        let mut res = vec![];
        let mut pacific = vec![vec![false; n as usize]; m as usize];
        let mut atlantic = vec![vec![false; n as usize]; m as usize];

        for j in 0..n {
            dfs(0, j, &heights, &mut pacific);
        }
        for i in 0..m {
            dfs(i, 0, &heights, &mut pacific);
        }
        for j in 0..n {
            dfs(m-1, j, &heights, &mut atlantic);
        }
        for i in 0..m {
            dfs(i, n-1, &heights, &mut atlantic);
        }
        let n = n as usize;
        let m = m as usize;
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }
}

fn dfs(r:i32, c:i32, heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) {
    if visited[r as usize][c as usize] {
        return;
    }
    visited[r as usize][c as usize] = true;
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dr, dc) in directions {
        let nr = r + dr;
        let nc = c + dc;
        if nr < 0 || nr >= heights.len() as i32 || nc < 0 || nc >= heights[0].len() as i32 {
            continue;
        }
        if heights[nr as usize][nc as usize] < heights[r as usize][c as usize] {
            continue;
        }
        dfs(nr, nc, heights, visited);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ],
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ])
        );
    }
}
