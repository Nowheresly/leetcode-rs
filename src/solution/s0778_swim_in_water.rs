use std::collections::BinaryHeap;

pub struct Solution {}

#[derive(Debug, Eq, PartialEq)]
pub struct Data {
    pub dist:i32,
    pub node: i32,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}
impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut heap:BinaryHeap<Data> = BinaryHeap::new();

        let mut dist = vec![i32::MAX; n*n];
        dist[0] = grid[0][0];

        heap.push(Data{dist:0, node:0});

        while heap.is_empty() == false {
            let top = heap.pop().unwrap();
            let u = top.node;

            let neighbours = find_neighbours(&grid, u);
            for neighbour in neighbours {
                let v = neighbour.node as usize;

                if dist[v] > dist[u as usize].max(neighbour.dist) {
                    dist[v] = dist[u as usize].max(neighbour.dist);

                    heap.push(Data{dist:dist[v], node:v as i32});
                }
            }
        }
        dist[n*n-1]
    }
}


fn find_neighbours(grid: &Vec<Vec<i32>>, u: i32) -> Vec<Data> {
    let mut res = vec![];
    let n = grid.len() as i32;
    let x = u / n;
    let y = u % n;
    let dx = vec![0, -1, 0, 1];
    let dy = vec![-1, 0, 1, 0];
    for i in 0..4 {
        let nx = x + dx[i];
        let ny = y + dy[i];
        if nx < 0 || nx >= n || ny < 0 || ny >= n {
            continue;
        }
        res.push(Data{dist:grid[nx as usize][ny as usize], node:nx*n+ny});
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::swim_in_water(vec![vec![0,2],vec![1,3]]));
        assert_eq!(16, Solution::swim_in_water(vec![vec![0,1,2,3,4],vec![24,23,22,21,5],vec![12,13,14,15,16],vec![11,17,18,19,20],vec![10,9,8,7,6]]));

    }
}
