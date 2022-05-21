mod littlepath;

use std::env;
use std::path::PathBuf;
use clap::{Parser, ArgEnum};
use path_absolutize::*;

/// Use little paths to address your files and directories.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Base directory for the search.
    #[clap(short, long, default_value = ".")]
    base_directory: String,

    /// WIP. Return all matching paths.
    #[clap(short, long, arg_enum, default_value_t = Mode::All)]
    mode: Mode,

    /// WIP. If there are multiple paths tied with the top match score, exit
    /// with an error code.
    #[clap(short, long)]
    ensure_single_match: bool,

    /// littlepath to expand.
    query: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    /// Return all matching paths.
    All,

    /// Return all matches tied with the top match score.
    AllTop,

    /// Resolve the tie for the top match by selecting the longest path.
    /// In case multiple paths exist with the same longest length,
    /// one of them is picked.
    Longest,

    /// Resolve the tie for the top match by selecting the shortest path.
    /// In case multiple paths exist with the same longest length,
    /// one of them is picked.
    Shortest,
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

    // TODO: Implement all the modes and flags mentioned above instead of always
    // printing all the matches.
    for candidate in littlepath::resolve(query, relative_to) {
        let absolute_candidate = candidate.path
            .absolutize()
            .unwrap();

        match absolute_candidate.to_str() {
            Some(value) => println!("{}", value),
            None => (),
        }
    }
}
