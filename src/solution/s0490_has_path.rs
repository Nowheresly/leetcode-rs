pub struct Solution {}

impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        let n = maze.len();
        let m = maze[0].len();
        let mut q = vec![start];
        let mut visited = vec![vec![false; m]; n];
        let dx = vec![0, 0, 1, -1];
        let dy = vec![1, -1, 0, 0];

        while let Some(pos) = q.pop() {
            let x = pos[0] as usize;
            let y = pos[1] as usize;
            if x == destination[0] as usize && y == destination[1] as usize {
                return true;
            }
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            for i in 0..4 {
                let mut nx = x as i32;
                let mut ny = y as i32;
                while nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 && maze[nx as usize][ny as usize] == 0 {
                    nx += dx[i];
                    ny += dy[i];
                }
                nx -= dx[i];
                ny -= dy[i];
                if visited[nx as usize][ny as usize] {
                    continue;
                }
                q.push(vec![nx, ny]);
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
        assert_eq!(
            false,
            Solution::has_path(
                vec![vec![0,0,1,0,0],vec![0,0,0,0,0],vec![0,0,0,1,0],vec![1,1,0,1,1],vec![0,0,0,0,0]],
                vec![0,4],
                vec![3,2]
            )
        );
    }
}
