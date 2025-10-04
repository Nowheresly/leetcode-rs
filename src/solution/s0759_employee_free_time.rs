use std::collections::BinaryHeap;
use std::cmp::Reverse;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


pub struct Solution {}

impl Solution {
    pub fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
        let mut res = vec![];

        let mut pq:BinaryHeap<Reverse<Interval>> = BinaryHeap::new();
        for intervals in schedule {
            for interval in intervals {
                pq.push(Reverse(interval));
            }
        }
        let mut tmp = pq.pop().unwrap().0;

        while pq.is_empty() == false {
            if tmp.end < pq.peek().unwrap().0.start {
                res.push(Interval::new(tmp.end, pq.peek().unwrap().0.start));
                tmp = pq.pop().unwrap().0;
                continue;
            }

            if tmp.end < pq.peek().unwrap().0.end {
                tmp = pq.pop().unwrap().0;
            } else {
                pq.pop();
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
        assert_eq!(
            vec![Interval::new(3, 4)],
            Solution::employee_free_time(vec![
                vec![Interval::new(1, 2), Interval::new(5, 6)],
                vec![Interval::new(1, 3)],
                vec![Interval::new(4, 10)]
            ])
        );
    }
}
