mod utils;

use crate::utils::repo_list_cloner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let location_path = &args[2];

    println!("File: {}", file_path);
    println!("Directory: {}", location_path);

    repo_list_cloner::get_repos(file_path, location_path);
}
