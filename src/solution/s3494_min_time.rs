pub struct Solution {}

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let m = mana.len();
        let mut done = vec![0i64; n]; // means time at which wizard i is free

        for potion in 0..m {
            for wizard in 0..n {
                if wizard == 0 {
                    done[wizard] += (skill[wizard] as i64) * (mana[potion] as i64);
                } else {
                    done[wizard] = std::cmp::max(done[wizard], done[wizard - 1]) + (skill[wizard] as i64) * (mana[potion] as i64);
                }
            }
            for wizard in (0..n-1).rev() {
                done[wizard] = done[wizard + 1] - (skill[wizard + 1] as i64) * (mana[potion] as i64);
            }
        }
        done[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(110, Solution::min_time(vec![1,5,2,4], vec![5,1,4,2]));
        assert_eq!(5, Solution::min_time(vec![1,1,1], vec![1,1,1]));
        assert_eq!(21, Solution::min_time(vec![1,2,3,4], vec![1,2]));

    }
}
