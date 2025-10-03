

pub struct Solution {}

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        if *(&date1.cmp(&date2).is_lt()) {
            return Self::days_between_dates(date2, date1);
        }
        let date1 = date1.split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let date2 = date2.split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (y1, m1, d1) = (date1[0], date1[1], date1[2]);
        let (y2, m2, d2) = (date2[0], date2[1], date2[2]);

        let dd1 = f(y1, m1, d1);
        let dd2 = f(y2, m2, d2);
        dd1 - dd2
    }
}

fn f(year: i32, month: i32, day:i32) -> i32 {
    let mut res = day;
    let leap = is_leap(year);
    let feb = 28 + if leap { 1 } else { 0 };
    if month == 2 {
        res += 31;
    } else if month == 3 {
        res += 31 + feb;
    } else if month == 4 {
        res += 31 + feb + 31;
    } else if month == 5 {
        res += 31 + feb + 31 + 30;
    } else if month == 6 {
        res += 31 + feb + 31 + 30 + 31;
    } else if month == 7 {
        res += 31 + feb + 31 + 30 + 31 + 30;
    } else if month == 8 {
        res += 31 + feb + 31 + 30 + 31 + 30 + 31;
    } else if month == 9 {
        res += 31 + feb + 31 + 30 + 31 + 30 + 31 + 31;
    } else if month == 10 {
        res += 31 + feb + 31 + 30 + 31 + 30 + 31 + 31 + 30;
    } else if month == 11 {
        res += 31 + feb + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31;
    } else if month == 12 {
        res += 31 + feb + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30;
    }

    for i in 1900..year {
        if is_leap(i) {
            res += 366;
        } else {
            res += 365;
        }
    }
    res
}

fn is_leap(year: i32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        //assert_eq!(15, Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()));
        assert_eq!(0, Solution::days_between_dates("2019-06-29".to_string(), "2019-06-29".to_string()));
    }
}
