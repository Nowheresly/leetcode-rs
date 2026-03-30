
pub struct Solution {}

impl Solution {
    pub fn winning_player(x: i32, y: i32) -> String {
        let mut alice = false;
        let mut x = x;
        let mut y = y;
        loop {
            x -= 1;
            if x < 0 {
                break;
            }
            y -=4;
            if y < 0 {
                break;
            }
            alice = ! alice;
        }
        if alice {
            return String::from("Alice");
        }
        String::from("Bob")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("Bob"), Solution::winning_player(4, 11));
    }
}
