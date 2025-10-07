use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut res = vec![-1; n];

        let mut map:HashMap<i32, i32> = HashMap::new();
        let mut dry_days:Vec<i32> = vec![];

        for i in 0..n {
            let lake = rains[i];
            if lake == 0 {
                dry_days.push(i as i32);
                continue;
            }

            if map.contains_key(&lake) == false {
                map.insert(lake, i as i32);
                continue;
            }
            let from = map[&lake];
            let to = i as i32;
            let mut ok = false;
            for j in 0..dry_days.len() {
                let dry_day = dry_days[j];
                if dry_day <= from {
                    continue;
                }
                if dry_day >= to {
                    return vec![];
                }
                dry_days.remove(j);
                map.insert(lake, i as i32);
                res[dry_day as usize] = lake;
                ok = true;
                break;
            }
            if ok == false {
                return vec![];
            }
        }
        while dry_days.len() > 0 {
            let dry_day = dry_days.pop().unwrap();
            res[dry_day as usize] = 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(vec![-1,-1,-1,-1], Solution::avoid_flood(vec![1,2,3,4]));
        assert_eq!(vec![-1,-1,2,1,-1,-1], Solution::avoid_flood(vec![1,2,0,0,2,1]));
        assert_eq!(vec![] as Vec<i32>, Solution::avoid_flood(vec![1,2,0,1,2]));
    }
}
