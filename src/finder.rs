use std::collections::VecDeque;
use std::io;
use std::path::PathBuf;

use crate::constants::IGNORE_PATHS;

pub fn find(query: &str, start: PathBuf, capacity: usize, to_lower: bool) -> io::Result<Vec<PathBuf>> {
    let mut dirs = VecDeque::from(vec![start]);
    let mut result =  Vec::with_capacity(capacity);

    while let Some(dir) = dirs.pop_front() {
        'outer: for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                let path_str = path.to_str();
                //println!("look'n at {}", path_str.unwrap());
                for pattern in IGNORE_PATHS {
                    if path_str.unwrap().contains(pattern) {
                        continue 'outer;
                    }
                }
                dirs.push_back(path.clone());
            }
            if let Some(name) = path.file_name() {
                let search_name  = name.to_str();
                if let Some(n) = search_name {
                    if query.is_empty() || n.contains(query) {
                        result.push(path.clone());
                    }
                }
            }
        }
    }
    Ok(result)
}
