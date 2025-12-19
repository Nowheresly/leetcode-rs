use std::collections::HashMap;

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for i in 0..n { p[i] = i; }

        UnionFind {
            parent: p,
            rank: vec![0; n],
        }
    }

    fn find(self: &mut Self, n: usize) -> usize {
        if self.parent[n] == n {
            return n;
        }
        self.parent[n] = self.find(self.parent[n]);
        self.parent[n]
    }

    fn union(self: &mut Self, n: usize, m: usize) {
        let p1 = self.find(n);
        let p2 = self.find(m);
        if p1 == p2 {
            return;
        }
        if self.rank[p1] > self.rank[p2] {
            self.parent[p2] = p1;
        } else if self.rank[p1] < self.rank[p2] {
            self.parent[p1] = p2;
        } else {
            self.parent[p2] = p1;
            self.rank[p1] += 1;
        }
    }

    fn reset(self: &mut Self, n: usize) {
        self.parent[n] = n;
        self.rank[n] = 0;
    }

    fn connected(self: &mut Self, n: usize, m: usize) -> bool {
        self.find(n) == self.find(m)
    }
}


pub struct Solution {}
impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut meetings = meetings;
        meetings.sort_by_key(|x| x[2]);

        let mut same_time_meetings = HashMap::new();
        for meeting in meetings {
            same_time_meetings.entry(meeting[2]).or_insert(vec![]).push((meeting[0] as usize, meeting[1] as usize));
        }
        let mut uf = UnionFind::new(n as usize);
        uf.union(0, first_person as usize);

        let mut times: Vec<_> = same_time_meetings.keys().collect();
        times.sort_unstable();

        for key in times {
            for meeting in &same_time_meetings[key] {
                uf.union(meeting.0, meeting.1);
            }

            for meeting in &same_time_meetings[key] {
                if ! uf.connected(meeting.0, 0) {
                    uf.reset(meeting.0);
                    uf.reset(meeting.1);
                }
            }
        }
        let mut ans = vec![];
        for i in 0..n {
            if uf.connected(i as usize, 0) {
                ans.push(i as i32);
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

        assert_eq!(vec![0,1,2,3,5], Solution::find_all_people(6, vec![vec![1,2,5],vec![2,3,8],vec![1,5,10]], 1));

    }
}
