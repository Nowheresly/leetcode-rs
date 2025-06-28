pub struct Solution {}

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let str = num.to_string();
        let str_vec: Vec<char> = str.chars().collect();

        let mut c = str_vec[0];
        for i in 0..str.len() {
            if str_vec[i] != '9' {
                c = str_vec[i];
                break;
            }
        }
        let max_str:String = str.chars().map(|x| if x == c {
            '9'
        } else {
            x
        }).collect();
        let max = max_str.parse::<i32>().unwrap();

        c = str_vec[0];
        for i in 0..str.len() {
            if str_vec[i] != '0' {
                c = str_vec[i];
                break;
            }
        }
        let min_str:String = str.chars().map(|x| if x == c {
            '0'
        } else {
            x
        }).collect();
        let min = min_str.parse::<i32>().unwrap();

        return max - min;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(99009, Solution::min_max_difference(11891));
        assert_eq!(99, Solution::min_max_difference(90));
    }
}