use regex::Regex;
mod constants;
use gitleaks_rs::{check_sequential, print_leaks};

fn main() {
    let regexes: Vec<_> = constants::PATTERNS
        .iter()
        .map(|(id, re)| (id, Regex::new(re).expect("Invalid regex string")))
        .collect();

    let file_content = std::fs::read_to_string("data/test_file.txt").expect("Unable to read file");

    let found_matches = check_sequential(regexes, &file_content);

    print_leaks(found_matches, &file_content);
}
