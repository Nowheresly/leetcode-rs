
pub struct Solution {}

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let mut count1 = 0;
        let mut count2 = 0;
        let mut min_flips = n as i32;
        let ch = s.chars().collect::<Vec<char>>();

        for i in 0..(2*n) {
            let curr = ch[i%n];

            let expected1 = if i % 2 == 0 {'1'} else {'0'};
            let expected2 = if i % 2 == 0 {'0'} else {'1'};

            if curr != expected1 {
                count1 += 1;
            }
            if curr != expected2 {
                count2 += 1;
            }

            if i >= n {
                let left = i - n ;
                let left_char = ch[left % n];

                let expected_left1 = if left % 2 == 0 {'1'} else {'0'};
                let expected_left2 = if left % 2 == 0 {'0'} else {'1'};

                if left_char != expected_left1 {
                    count1 -= 1;
                }
                if left_char != expected_left2 {
                    count2 -= 1;
                }

                if i >= n -1 {
                    min_flips = min_flips.min(count1.min(count2));
                }
            }
        }

        min_flips
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2,  Solution::min_flips(String::from("111000")));
    }
}
