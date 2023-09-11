use clap::Parser;

mod finder;
mod constants;

use finder::find;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    ///The glob pattern to check
    pattern: String,

    ///The path to start searching
    path: std::path::PathBuf,

    ///Check hidden directories
    #[arg(short('i'), long, default_value_t = false)]
    insensative: bool,
    
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    //TODO: Get some parallelism in here
    //Run our finder program over the requested path and print the results out to stdout
    for path in find(&args.pattern, args.path, usize::from(args.count), args.insensative)? {
        if let Some(p) = path.to_str() {
            println!("{}", p);
        }
    }
    Ok(())
}
