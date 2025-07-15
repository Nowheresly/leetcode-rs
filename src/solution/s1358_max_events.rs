use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut max_day = 0;
        for event in &events {
            if event[1] > max_day {
                max_day = event[1];
            }
        }
        let mut evts = events.clone();
        evts.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut pq = BinaryHeap::new();
        let mut ret = 0;
        let mut i = 0;
        for day in 0..=max_day+1 {
            // Add all events that start on this day to the priority queue
            while i < evts.len() && evts[i][0] <= day {
                pq.push(Reverse(evts[i][1]));
                i += 1;
            }

            // Remove all events that end before today
            while let Some(&Reverse(end)) = pq.peek() {
                if end < day {
                    pq.pop();
                } else {
                    break;
                }
            }

            // If there are any events in the priority queue, attend the one that ends the earliest
            if let Some(&end) = pq.peek() {
                pq.pop();
                ret += 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        //assert_eq!(3, Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]));
        assert_eq!(4, Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]));
    }
}
