use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Data {
    pub task_id: i32,
    pub user_id: i32,
    pub priority: i32,
}
impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd<Self> for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.priority.cmp(&other.priority) == std::cmp::Ordering::Equal {
            return Some(self.task_id.cmp(&other.task_id))
        }
        Some(self.priority.cmp(&other.priority))
    }
}


struct TaskManager {
    map: BTreeMap<Data, i32>,
    map_task_user: HashMap<i32, i32>,
    map_task_priority: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {

    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut map_task_priority = HashMap::new();
        let mut map_task_user = HashMap::new();
        let mut map = BTreeMap::new();

        for task in tasks {
            let user_id = task.get(0).unwrap();
            let task_id = task.get(1).unwrap();
            let priority = task.get(2).unwrap();
            map_task_priority.insert(task_id.clone(), priority.clone());
            map_task_user.insert(task_id.clone(), user_id.clone());
            map.insert(Data { task_id: task_id.clone(), user_id: user_id.clone(), priority: priority.clone() }, user_id.clone());
        }

        Self {
            map,
            map_task_user,
            map_task_priority,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.map.insert(Data { task_id: task_id.clone(), user_id: user_id.clone(), priority: priority.clone() }, user_id.clone());
        self.map_task_user.insert(task_id.clone(), user_id.clone());
        self.map_task_priority.insert(task_id.clone(), priority.clone());
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let user_id = self.map_task_user.get(&task_id).unwrap();
        let priority = self.map_task_priority.get(&task_id).unwrap();
        self.map.remove(&Data { task_id: task_id.clone(), user_id: user_id.clone(), priority: priority.clone() });
        self.map.insert(Data { task_id: task_id.clone(), user_id: user_id.clone(), priority: new_priority.clone() }, user_id.clone());
        self.map_task_priority.insert(task_id, new_priority);
    }

    fn rmv(&mut self, task_id: i32) {
        let user_id = self.map_task_user.get(&task_id).unwrap();
        let priority = self.map_task_priority.get(&task_id).unwrap();
        self.map.remove(&Data { task_id: task_id.clone(), user_id: user_id.clone(), priority: priority.clone() });
        self.map_task_user.remove(&task_id);
        self.map_task_priority.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        if self.map.is_empty() {
            return -1;
        }
        let data = self.map.pop_last().unwrap();

        let user_id = data.1;
        let key = data.0;

        self.map.remove(&key);
        self.map_task_user.remove(&key.task_id);
        self.map_task_priority.remove(&key.task_id);
        user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut tm = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        tm.add(4, 104, 5);
        tm.edit(102, 8);
        let user = tm.exec_top();
        assert_eq!(3, user);
        tm.rmv(101);
        tm.add(5, 105, 15);
        let user = tm.exec_top();
        assert_eq!(5, user);
    }
}