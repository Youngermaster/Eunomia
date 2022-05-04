use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

pub fn get_repos(file_path: &String, location_path: &String) {
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                clone_repo(ip, location_path);
            }
        }
    }
}

pub fn clone_repo(url: String, location_path: &String) {
    Command::new("git")
        .arg("clone")
        .arg(&url)
        .current_dir(location_path)
        .spawn()
        .expect("failed to execute process");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
