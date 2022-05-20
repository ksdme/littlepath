mod littlepath;

use std::env;
use std::fs;
use std::vec;
use std::path::Path;
use std::path::PathBuf;
use path_absolutize::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The littlepath in question to be expanded.
    let query_option = args.get(1).unwrap();
    let query = PathBuf::from(query_option);

    // The directory relative to which the little path expansion should happen,
    // defaults to the working directory of the shell this program is invoked
    // from.
    let default_relative_to = ".".to_owned();
    let relative_to_option = args.get(2)
        .unwrap_or(&default_relative_to);
    let relative_to = PathBuf::from(relative_to_option)
        .absolutize_from(&env::current_dir().unwrap())
        .unwrap()
        .to_path_buf();

    littlepath::resolve(query, relative_to);
}
