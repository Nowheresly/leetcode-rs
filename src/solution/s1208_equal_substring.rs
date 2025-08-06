
pub struct Solution {}

impl Solution {
    pub fn equal_substring(s: String, t : String, max_cost: i32) -> i32 {
        let mut ret:i32 = 0;
        let diff = s.chars().zip(t.chars())
            .map(|(c1, c2)| (c1 as i32 - c2 as i32).abs())
            .collect::<Vec<i32>>();

        let mut left = 0;
        let mut cost = 0;
        for right in 0..diff.len() {
            cost += diff[right];
            if cost <= max_cost {
                ret = ret.max(right as i32 - left + 1);
            } else {
                while cost > max_cost {
                    if left == right as i32 {
                        cost = 0;
                        left += 1;
                        break;
                    }
                    cost -= diff[left as usize];
                    left += 1;
                }
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

        assert_eq!(3, Solution::equal_substring(String::from("abcd"), String::from("bcdf"), 3));
        assert_eq!(1, Solution::equal_substring(String::from("abcd"), String::from("cdef"), 3));
        assert_eq!(1, Solution::equal_substring(String::from("abcd"), String::from("acde"), 0));
    }
}
