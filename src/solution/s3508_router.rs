use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Data {
    timestamp: i32,
    source: i32,
    destination: i32,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}



struct Router {
    memory_limit: i32,
    queue: Vec<Data>,
    duplicated: HashSet<Data>,
    map : HashMap<i32, Vec<Data>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {

    fn new(memory_limit: i32) -> Self {
        Self {
            memory_limit,
            queue: vec![],
            duplicated: HashSet::new(),
            map: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let data = Data {
           timestamp,
            source,
            destination,
        };

        if self.duplicated.contains(&data) {
            return false;
        }
        while self.queue.len() >= self.memory_limit as usize {
            let data = self.queue.remove(0);
            self.duplicated.remove(&data);
            self.map.get_mut(&data.destination).unwrap().remove(0);
        }
        self.queue.push(data.clone());
        self.duplicated.insert(data.clone());
        if self.map.contains_key(&destination) == false{
            self.map.insert(destination, vec![data.clone()]);
        } else {
            let l = self.map.get_mut(&destination).unwrap();
            l.push(data);
        }
        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if self.queue.is_empty() {
            return vec![];
        }
        let d = self.queue.remove(0);
        self.duplicated.remove(&d);
        self.map.get_mut(&d.destination).unwrap().remove(0);
        vec![d.source, d.destination, d.timestamp]
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let empty_vec = vec![];
        let list = self.map.get(&destination).unwrap_or(&empty_vec);
        let mut start:i32 = 0;
        let mut end:i32 = list.len() as i32 - 1;
        let mut begin:i32 = -1;

        while start <= end {
            let mid = (start + end) / 2;
            if list[mid as usize].timestamp == start_time {
                begin = mid;
                end = mid - 1;
                continue;
            }
            if list[mid as usize].timestamp < start_time {
                start = mid + 1;
            } else {
                begin = mid;
                end = mid - 1;
            }
        }
        if begin == -1 {
            return 0;
        }
        start = begin;
        end = list.len() as i32 - 1;
        let mut finish:i32 = -1;
        while start <= end {
            let mid = (start + end) / 2;
            if list[mid as usize].timestamp == end_time {
                finish = mid;
                start = mid + 1;
                continue;
            }
            if list[mid as usize].timestamp < end_time {
                finish = mid as i32;
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        if finish == -1 {
            return 0;
        }

        finish - begin + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut router = Router::new(3);
        router.add_packet(1, 4, 90);
        router.add_packet(2, 5, 90);
        router.add_packet(1, 4, 90);
        router.add_packet(3, 5, 95);
        router.add_packet(4, 5, 105);
        assert_eq!(vec![2,5,90], router.forward_packet());
        router.add_packet(5, 2, 110);
        assert_eq!(1, router.get_count(5, 100, 110));
    }

    #[test]
    fn test_2() {
        let mut router = Router::new(3);
        router.add_packet(1, 4, 6);
        assert_eq!(0, router.get_count(4, 1, 4));
    }
}
