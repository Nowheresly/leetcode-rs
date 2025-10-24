
pub struct Solution {}

pub struct Data {
    min:i32
}

impl Data {
    fn dfs(&mut self, n:i32, sb:String, available: Vec<i32>) {
        if available.is_empty() {
            let tmp:i32 = sb.parse().unwrap();
            if tmp > n {
                self.min = self.min.min(tmp);
            }
            return;
        }
        let mut prev = -1;
        for i in available.iter() {
            if *i == prev {
                continue;
            }
            prev = *i;
            let c = std::char::from_digit(*i as u32, 10).unwrap();
            let mut new_sb = sb.clone();
            new_sb.push(c);
            let mut new_available = available.clone();
            let pos = new_available.iter().position(|x| *x == *i).unwrap();
            new_available.remove(pos);
            self.dfs(n, new_sb, new_available);

        }
    }
}

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let sets = vec![
            vec![1],
            vec![1,2],
            vec![1,3],
            vec![1,4],
            vec![1,5],
            vec![1,6],
            vec![1,2,3],
            vec![1,2,4],
            vec![2],
            vec![2,3],
            vec![2,4],
            vec![2,5],
            vec![3],
            vec![3,4],
            vec![4],
            vec![5],
            vec![6],
            vec![7],
        ];
        let min_len = n.to_string().len();
        let mut data = Data {min: i32::MAX};
        for list in sets {
            let mut size = 0;
            for i in list.iter() {
                size += i;
            }
            if size < min_len {
                continue;
            }
            if size > min_len + 1 {
                continue;
            }
            let sb = String::new();
            let mut available = vec![];
            for cand in list.iter() {
                let mut tmp = *cand as i32;
                while tmp > 0 {
                    available.push(*cand as i32);
                    tmp -= 1;
                }
            }

            data.dfs(n, sb, available);
        }
        data.min
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(22, Solution::next_beautiful_number(1));

    }
}
