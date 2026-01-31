pub struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut beg:i32 = 0;
        let mut end:i32 = letters.len() as i32 - 1;
        let mut ret = -1;
        while beg <= end {
            let mid = beg + (end - beg) / 2;
            if target < letters[mid as usize] {
                ret = mid;
                end = mid - 1;
            } else {
                beg = mid + 1
            }
        }
        if ret == -1 {
            letters[0]
        } else {
            letters[ret as usize]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!('c', Solution::next_greatest_letter(vec!['c','f','j'], 'a'));
    }
}
