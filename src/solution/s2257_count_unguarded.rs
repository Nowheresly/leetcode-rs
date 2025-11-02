use std::collections::VecDeque;

pub struct Solution {}

pub struct Data {
    pub x: usize,
    pub y: usize,
    pub dx: i32,
    pub dy: i32
}
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let dx = vec![-1, 1, 0, 0];
        let dy = vec![0, 0, -1, 1];
        // 0: unguarded
        // 1: guard
        // 2: wall
        // 3: guarded
        let mut guarded = vec![vec![0; n as usize]; m as usize];

        let mut queue:VecDeque<Data> = std::collections::VecDeque::new();

        for guard in guards {
            for i in 0..4 {
                let nextx = guard[0] + dx[i];
                let nexty = guard[1] + dy[i];
                if nextx < 0 || nextx >= m || nexty < 0 || nexty >= n {
                    continue;
                }
                queue.push_back(Data { x: nextx as usize, y: nexty as usize, dx: dx[i], dy: dy[i] });
            }
            guarded[guard[0] as usize][guard[1] as usize] = 1;
        }

        for wall in walls {
            guarded[wall[0] as usize][wall[1] as usize] = 2;
        }

        while queue.is_empty() == false {
            let data = queue.pop_front().unwrap();
            let x = data.x as i32;
            let y = data.y as i32;
            if x < 0 || x >= m || y < 0 || y >= n {
                continue;
            }
            if guarded[x as usize][y as usize] == 2 || guarded[x as usize][y as usize] == 1 {
                continue;
            }
            guarded[x as usize][y as usize] = 3;
            let nextx = x + data.dx;
            let nexty = y + data.dy;
            queue.push_back(Data { x: nextx as usize, y: nexty as usize, dx: data.dx, dy: data.dy });
        }
        let mut ans = 0;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if guarded[i][j] == 0 {
                    ans += 1;
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
        assert_eq!(7, Solution::count_unguarded(4,6,vec![vec![0,0],vec![1,1],vec![2,3]], vec![vec![0,1],vec![2,2],vec![1,4]]));
        assert_eq!(4, Solution::count_unguarded(3,3,vec![vec![1,1]], vec![vec![0,1],vec![1,0],vec![1,2],vec![2,1]]));

    }
}
