use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut pp = BinaryHeap::new();
        for i in players {
            pp.push(Reverse(i));
        }
        let mut pt = BinaryHeap::new();
        for i in trainers {
            pt.push(Reverse(i));
        }
        let mut ret = 0;
        while pp.is_empty() == false && pt.is_empty() == false {
            let p = pp.pop().unwrap();

            while pt.peek().unwrap().cmp(&p) == std::cmp::Ordering::Greater {
                pt.pop();
                if pt.is_empty() {
                    return ret;
                }
            }

            ret += 1;
            pt.pop();
        }
        return ret;
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::match_players_and_trainers(vec![4,7,9], vec![8,2,5,8]));
        assert_eq!(1, Solution::match_players_and_trainers(vec![1,1,1], vec![10]));
    }
}