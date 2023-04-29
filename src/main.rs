use clap::Parser;
use std::collections::VecDeque;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    ///The glob pattern to check
    pattern: String,

    ///The path to start searching
    path: std::path::PathBuf,

    ///Check hidden directories
    #[arg(short, long, default_value_t = false)]
    hidden: bool,
    
    ///The maximum number of results to report
    #[arg(short, long, default_value_t = 500)]
    count: u16,
}

fn find(query: &str, start: PathBuf) -> io::Result<Vec<PathBuf>> {
    //let start = PathBuf::from(start); //was OsStr, not sure what the difference is
    let mut dirs = VecDeque::from(vec![start]);
    let mut result = Vec::new();

    while let Some(dir) = dirs.pop_front() {
        for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                dirs.push_back(path.clone());
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

    for path in find(&args.pattern, args.path)? {
        if let Some(p) = path.to_str() {
            println!("{}", p);
        }
    }
    Ok(())
}
