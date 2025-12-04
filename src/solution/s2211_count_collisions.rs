pub struct Solution {}

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let n = directions.len() as i32;
        let mut l:i32 = 0;
        let mut r:i32 = n as i32 - 1;
        let directions = directions.chars().collect::<Vec<char>>();
        while l < n && directions[l as usize] == 'L' {
            l += 1;
        }

        while r >= l && directions[r as usize] == 'R' {
            r -= 1;
        }
        let mut ans = 0;
        for i in l..=r {
            if directions[i as usize] != 'S' {
                ans += 1;
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
        assert_eq!(5, Solution::count_collisions(String::from("RLRSLL")));
        assert_eq!(0, Solution::count_collisions(String::from("LLRR")));
    }
}
