pub struct Solution {}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; n as usize];
        let mut busy = vec![0i64; n as usize];
        let mut meetings = meetings;
        meetings.sort_by(|a, b| a[0].cmp(&b[0]));

        for meeting in meetings {
            let start = meeting[0] as usize;
            let end = meeting[1] as usize;
            let mut earliest = i64::MAX;
            let mut room_index = -1i32;
            let mut assigned = false;

            for i in 0..n {
                if busy[i as usize] < earliest {
                    earliest = busy[i as usize];
                    room_index = i;
                }
                if busy[i as usize] <= start as i64 {
                    count[i as usize] += 1;
                    busy[i as usize] = end as i64;
                    assigned = true;
                    break;
                }
            }
            if assigned == false {
                count[room_index as usize] += 1;
                busy[room_index as usize] += (end - start) as i64;
            }
        }
        let mut max = 0;
        let mut res = 0;
        for i in 0..n {
            if count[i as usize] > max {
                max = count[i as usize];
                res = i;
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
            0,
            Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]])
        );
    }
}
