mod littlepath;

use std::env;
use std::path::PathBuf;
use clap::Parser;
use path_absolutize::*;

/// Use little paths to address your files and directories.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Base directory for the search.
    #[clap(short, long, default_value = ".")]
    base_directory: String,

    /// littlepath to expand.
    query: String,
}

fn main() {
    let args = Args::parse();

    // The littlepath in question to be expanded.
    let query = PathBuf::from(args.query);

    // The directory relative to which the little path expansion should happen,
    // defaults to the working directory of the shell this program is invoked
    // from.
    let relative_to = PathBuf::from(args.base_directory)
        .absolutize_from(&env::current_dir().unwrap())
        .unwrap()
        .to_path_buf();

    littlepath::resolve(query, relative_to);
}
