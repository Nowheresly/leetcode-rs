pub struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut next_zeroes = vec![0; n];
        next_zeroes[n - 1] = n;
        for i in (0..n - 1).rev() {
            next_zeroes[i] = if chars[i+1] == '0' {
                i + 1
            } else {
                next_zeroes[i + 1]
            };
        }
        let mut ans: i32 = 0;
        for l in 0..n {
            let mut zeroes = if chars[l] == '0' { 1 } else { 0 };
            let mut r = l;

            while zeroes * zeroes <= n {
                let next = next_zeroes[r];
                let ones = (next - l) - zeroes;
                if ones >= zeroes * zeroes {
                    ans += (next - r).min(ones - zeroes * zeroes + 1) as i32;
                }
                r = next;
                zeroes += 1;
                if r == n {
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::number_of_substrings(String::from("00011")));
    }
}
