use regex::Regex;
use std::time::Instant;

fn main() {
    let patterns = [
        (1,r"abc"),
        (2,r"https?://.+:.+@dev.azure.com.*"),
        (3,r"https?://.+:.+@dev.azure.com.*"),
        (4,r"eyj[a-zA-Z0-9\-_%]+\.eyj[a-zA-Z0-9\-_%]+\.[a-zA-Z0-9\-_%]+"), // OAuth JWT
        (5,r"[tT]oken")
    ];

    let regexes: Vec<_> = patterns
        .iter()
        .map(|(id,re)| (id,Regex::new(re).expect("Invalid regex string")))
        .collect();

    // read file 
    let start = Instant::now();
    let file_content = std::fs::read_to_string("data/test_file.txt").expect("Unable to read file");
    println!("Reading file took: {:?}", start.elapsed());

    // check for patterns
    let start = Instant::now();
    for (id, regex) in regexes.iter() {
        if regex.is_match(&file_content) {
            // let found_match = regex.find(&file_content).unwrap(); // limitation: only first match, no surrounding text
            let found_matches = regex.find_iter(&file_content).map(|m| m.as_str()).collect::<Vec<&str>>(); // get all matches
            println!("Found pattern '{}' {} times: {:?}", id, found_matches.len(), found_matches);
        }
    }
    println!("Checking for patterns took: {:?}", start.elapsed());

}

/*
 * First iteration: function to identify 5 common patterns in strings - check
 * Second itegration: function to read a file and check for patterns, get line number and line text
 * Third iteration: build CLI to read all files in a directory and check for patterns
 * Fourth iteration: CLI to read all files in a directory and check for patterns, with exclusion list
 * Fifth iteration: check git log for patterns
 *
 * Feature list:
 * regex pattern matching
 * false positive exlusion
 * filepath & pre-fix exclusion
 *
*/
