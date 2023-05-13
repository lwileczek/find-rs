use clap::Parser;
use std::collections::VecDeque;
use std::io;
use std::path::PathBuf;

mod constants;

use crate::constants::IGNORE_PATHS;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    ///The glob pattern to check
    pattern: String,

    ///The path to start searching
    path: std::path::PathBuf,

    ///Check hidden directories
    #[arg(short('s'), long, default_value_t = false)]
    hidden: bool,
    
    ///The maximum number of results to report
    #[arg(short, long, default_value_t = 512)]
    count: u16,

    ///The maximum layors of depth to searth within one directory
    #[arg(short, long, default_value_t = 32)]
    depth: u16,

    ///Print out which directory we're looking at or not
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}


fn find(query: &str, start: PathBuf, capacity: usize, check_hidden: bool) -> io::Result<Vec<PathBuf>> {
    //let start = PathBuf::from(start); //was OsStr, not sure what the difference is
    let mut dirs = VecDeque::from(vec![start]);
    let mut result =  Vec::with_capacity(capacity);

    while let Some(dir) = dirs.pop_front() {
        'outer: for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                if check_hidden {
                    dirs.push_back(path.clone());
                    continue;
                }
                let path_str = path.to_str();
                //println!("look'n at {}", path_str.unwrap());
                for pattern in IGNORE_PATHS {
                    if path_str.unwrap().contains(pattern) {
                        continue 'outer;
                    }
                }
                if !IGNORE_PATHS.contains(&path_str.unwrap()) {
                    dirs.push_back(path.clone());
                }
            }
            if let Some(name) = path.file_name() {
                if query.is_empty() || query == name {
                    result.push(path.clone());
                }
            }
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    for path in find(&args.pattern, args.path, usize::from(args.count), args.hidden)? {
        if let Some(p) = path.to_str() {
            println!("{}", p);
        }
    }
    Ok(())
}
