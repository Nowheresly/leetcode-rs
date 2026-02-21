pub struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }
        let mut climb = true;
        for i in 1..n {
            let diff = arr[i] - arr[i-1];
            if diff == 0 {
                return false;
            }
            if diff > 0 && climb == false {
                return false;
            }
            if diff < 0 && climb {
                if i == 1 {
                    // need to climb first
                    return false;
                }
                // we found peak
                climb = false;
                continue;
            }
        }
        if climb {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::valid_mountain_array(vec![3, 5, 5]));
        assert_eq!(true, Solution::valid_mountain_array(vec![0,3,2,1]));
    }
}
