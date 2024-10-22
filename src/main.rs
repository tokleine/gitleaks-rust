use regex::Regex;
// use std::time::Instant;

fn main() {
    //define patterns
    let patterns = [
        (1, r"abc"),
        (2, r"https?://.+:.+@dev.azure.com.*"),
        (3, r"https?://.+:.+@dev.azure.com.*"),
        (
            4,
            r"eyj[a-zA-Z0-9\-_%]+\.eyj[a-zA-Z0-9\-_%]+\.[a-zA-Z0-9\-_%]+",
        ), // OAuth JWT
        (5, r"[tT]oken"),
    ];

    const LOOK_AHEAD_AND_BEHIND_SIZE: usize = 5;

    // compile regexes, slow operation
    let regexes: Vec<_> = patterns
        .iter()
        .map(|(id, re)| (id, Regex::new(re).expect("Invalid regex string")))
        .collect();

    // read file
    // let start = Instant::now();
    let file_content = std::fs::read_to_string("data/test_file.txt").expect("Unable to read file");
    // println!("Reading file took: {:?}", start.elapsed());

    // check for patterns without itering over lines
    // let start = Instant::now();
    for (id, regex) in regexes.iter() {
        if regex.is_match(&file_content) {
            // let found_match = regex.find(&file_content).unwrap(); // limitation: only first match, no surrounding text
            let found_matches: Vec<(usize, usize, &str)> = regex
                .find_iter(&file_content)
                .map(|m| (m.start(), m.end(), m.as_str())) // get match as well as start and end index
                .collect();

            // get lines of found matches and add to found_matches
            let found_lines: Vec<usize> = found_matches
                .iter()
                .map(|(start_idx, _, _)| file_content[..*start_idx].matches('\n').count() + 1)
                .collect();


            // add lines to found_matches
            let found_matches: Vec<(usize, usize, &str, usize)> = found_matches
                .iter()
                .zip(found_lines.iter())
                .map(|((start_idx, end_idx, s), line)| {
                    (
                        *start_idx,
                        *end_idx,
                        *s,
                        *line,
                    )
                })
                .collect();

            // print found matches
            for ( start_idx, end_idx, _, line) in found_matches {
                println!(
                    "Found pattern '{}' at line {}: ...{}{}{}{}{}...",
                    id,
                    line,
                    &file_content[start_idx - LOOK_AHEAD_AND_BEHIND_SIZE..start_idx]
                        .replace("\n", ""),
                    "\x1b[1;31m", // ANSI escape code for bold red text
                    &file_content[start_idx..end_idx].replace("\n", ""),
                    "\x1b[0m", // ANSI escape code to reset
                    &file_content[end_idx..end_idx + LOOK_AHEAD_AND_BEHIND_SIZE].replace("\n", "")
                );
            }
        }
    }
    // println!("Checking for patterns took: {:?}", start.elapsed());
}
