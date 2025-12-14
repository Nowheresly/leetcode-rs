
pub struct Solution {}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let modu = 1_000_000_007;
        let mut seat = 0;
        let mut ans:i64 = 1;
        let mut dispo = 1;
        let mut count_seats = 0;
        for c in corridor.chars() {
            if c == 'P' && seat < 2 {
                continue;
            }
            if c == 'P' && seat == 2 {
                dispo += 1;
                continue;
            }
            if c == 'S' && seat == 0 {
                count_seats += 1;
                seat += 1;
                continue;
            }
            if c == 'S' && seat == 1 {
                count_seats += 1;
                seat += 1;
                dispo = 1;
                continue;
            }
            if c == 'S' && seat == 2 {
                count_seats += 1;
                seat = 1;
                ans *= dispo;
                ans %= modu;
                dispo = 1;
            }
        }
        if count_seats % 2 == 1 || count_seats == 0 {
            return 0;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::number_of_ways(String::from("SSPPSPS")));

    }
}
