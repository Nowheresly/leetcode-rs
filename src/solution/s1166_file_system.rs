use std::collections::HashMap;

type FsNode = Box<HashMap<String, FsItem>>;
// This is the recursive enum that represents either a directory or a file.
enum FsItem {
    End,
    Directory(FsNode),
}

struct FileSystem {
    root: FsNode,
    value_of: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSystem {

    fn new() -> Self {
        FileSystem {
            root: Box::new(HashMap::new()),
            value_of: HashMap::new(),
        }
    }

    fn create_path(&mut self, path: String, value: i32) -> bool {
        let elems: Vec<&str> = path.split('/').collect();
        let mut curr = &mut self.root;
        for i in 0..elems.len() {
            let is_final = i == elems.len()-1;
            let elem = elems[i];
            if elem.is_empty() {
                continue;
            }
            if is_final {
                if curr.contains_key(elem) {
                    return false;
                }
                curr.insert(elem.to_string(), FsItem::Directory(Box::new(HashMap::new())));
                self.value_of.insert(
                    path.clone(),
                    value
                );
                return true;
            }

            if let Some(FsItem::Directory(next_dir)) = curr.get_mut(elem) {
                // Update curr to point to the next directory
                curr = next_dir;
            } else {
                return false; // If it's not a directory, we can't continue.
            }
        }
        false
    }

    fn get(&self, path: String) -> i32 {
        self.value_of.get(&path).cloned().unwrap_or(-1)
    }
}

/**
 * Your FileSystem object will be instantiated and called as such:
 * let obj = FileSystem::new();
 * let ret_1: bool = obj.create_path(path, value);
 * let ret_2: i32 = obj.get(path);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut fs = FileSystem::new();
        assert_eq!(true, fs.create_path("/a".to_string(), 1));
        assert_eq!(1, fs.get("/a".to_string()));
    }
}
