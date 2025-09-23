
pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let vers1 = version1.split('.').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let vers2 = version2.split('.').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for i in 0..vers1.len().max(vers2.len()) {
            let v1 = vers1.get(i).unwrap_or(&0);
            let v2 = vers2.get(i).unwrap_or(&0);
            if v1 > v2 {
                return 1;
            } else if v1 < v2 {
                return -1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(-1, Solution::compare_version(String::from("1.2"), String::from("1.10")));
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.01"), String::from("1.001"))
        )
    }
}
