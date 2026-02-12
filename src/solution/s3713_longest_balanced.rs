pub struct Solution {}

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let nums:Vec<char> = s.chars().collect();
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        let mut res = 0i32;
        for i in 0..(n-1) {
            let mut map = vec![0;26];
            map[nums[i] as usize - 'a' as usize] += 1;
            for j in (i+1)..n {
                map[nums[j] as usize - 'a' as usize] += 1;
                let x = map[nums[j] as usize - 'a' as usize];
                let mut good = true;
                for k in 0..26 {
                    if map[k] > 0 && map[k] != x {
                        good = false;
                        break;
                    }
                }
                if good {
                    res = res.max(j as i32 - i as i32 + 1);
                }
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
        assert_eq!(4, Solution::longest_balanced(String::from("abbac")));
        assert_eq!(4, Solution::longest_balanced(String::from("zzabccy")));
    }
}
