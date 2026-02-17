pub struct Solution {}

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut res = vec![];
        for i in 0i32..1024 {
            let h = i >> 6;
            let m = i & 0x3f;
            if h < 12 && m < 60 && i.count_ones() == turned_on as u32 {
                res.push(format!("{:0>1}:{:0>2}", h, m));
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
        assert_eq!(
            vec![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"
            ],
            Solution::read_binary_watch(1)
        );
    }
}
