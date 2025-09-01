use std::collections::BinaryHeap;
use std::cmp::Ordering;
pub struct Solution {}

#[derive(Debug, PartialEq)]
struct Data {
    pass: f64,
    total: f64,
}
impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ra = (self.pass + 1.0) / (self.total + 1.0) - (self.pass / self.total);
        let rb = (other.pass + 1.0) / (other.total + 1.0) - (other.pass / other.total);
        ra.partial_cmp(&rb)
    }
}

impl Eq for Data {}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut pq:BinaryHeap<Data> = BinaryHeap::new();

        for (_i, c) in classes.iter().enumerate() {
            let pass = c[0] as f64;
            let total = c[1] as f64;
            pq.push(Data { pass, total });
        }
        let mut es = extra_students as usize;
        while es > 0 {
            let d = pq.pop().unwrap();
            es -= 1;
            let pass = d.pass + 1.0;
            let total = d.total + 1.0;
            pq.push(Data { pass, total });
        }

        let mut res = 0.0;
        while let Some(d) = pq.pop() {
            res += d.pass / d.total;
        }
        res / classes.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0.78333f64, ( Solution::max_average_ratio(vec![vec![1,2],vec![3,5],vec![2,2]], 2) * 100_000.0).round() / 100_000.0);

        assert_eq!(0.53485f64, (Solution::max_average_ratio(vec![vec![2,4],vec![3,9],vec![4,5],vec![2,10]], 4) * 100_000.0).round() / 100_000.0);
    }
}
