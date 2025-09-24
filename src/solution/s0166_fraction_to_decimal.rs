use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut negative = false;
        if numerator < 0 && denominator > 0 || numerator > 0 && denominator < 0 {
            negative = true;
        }
        let mut numerator:i64 = numerator as i64;
        let mut denominator:i64 = denominator as i64;
        numerator = numerator.abs();
        denominator = denominator.abs();
        let mut start = String::new();
        if negative {
            start.push('-');
        }
        let st = numerator / denominator;
        start.push_str(&st.to_string());
        let mut end = String::new();
        let mut remain = numerator  - (st * denominator);

        let mut remainers:HashSet<i64> = HashSet::new();
        remainers.insert(remain);
        let mut index:Vec<i64> = vec![];
        index.push(remain);
        let mut parent = false;
        while remain != 0 {
            remain = remain * 10;
            let div = remain / denominator;
            end.push_str(&div.to_string());

            remain = remain - (div * denominator);
            if remainers.contains(&remain) {
                parent = true;
                break;
            }
            remainers.insert(remain);
            index.push(remain);
        }
        if end.len() > 0 {
            if parent {
                let idx = index.iter().position(|&r| r == remain).unwrap();
                start.push('.');
                start.push_str(&end[0..idx]);
                start.push('(');
                start.push_str(&end[idx..end.len()]);
                start.push(')');

                return start;
            }
            start.push('.');
            start.push_str(&end);
            return start;
        }

        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("0.5"), Solution::fraction_to_decimal(1, 2));
        assert_eq!(String::from("2"), Solution::fraction_to_decimal(2, 1));
        assert_eq!(String::from("0.(012)"), Solution::fraction_to_decimal(4, 333));
    }
}
